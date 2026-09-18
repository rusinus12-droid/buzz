import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import {
  createTeam,
  deleteTeam,
  listTeams,
  updateTeam,
} from "@/shared/api/tauriTeams";
import type {
  AgentTeam,
  CreateTeamInput,
  UpdateTeamInput,
} from "@/shared/api/types";

export const teamsQueryKey = ["teams"] as const;

export function useTeamsQuery() {
  const queryClient = useQueryClient();

  return useQuery({
    queryKey: teamsQueryKey,
    queryFn: async () => {
      const teams = await listTeams();
      // The Solo Dev fork seeds its two editable persona definitions while
      // loading teams. If personas were cached first, refresh that cache now so
      // the just-seeded Architect/Implementer become visible in the same UI
      // session instead of appearing as missing team members until restart.
      await queryClient.invalidateQueries({ queryKey: ["personas"] });
      return teams;
    },
    staleTime: 30_000,
    // No refetchInterval: inbound relay team changes emit `agents-data-changed`
    // (handled by useAgentsDataRefresh). Same redundant-poll removal as
    // usePersonasQuery.
  });
}

export function useCreateTeamMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: CreateTeamInput) => createTeam(input),
    onSuccess: (created) => {
      queryClient.setQueryData<AgentTeam[]>(teamsQueryKey, (current) => {
        const next = current ?? [];
        return [created, ...next.filter((team) => team.id !== created.id)];
      });
    },
    onSettled: async () => {
      await queryClient.invalidateQueries({ queryKey: teamsQueryKey });
    },
  });
}

export function useUpdateTeamMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: UpdateTeamInput) => updateTeam(input),
    onSettled: async () => {
      await queryClient.invalidateQueries({ queryKey: teamsQueryKey });
    },
  });
}

export function useDeleteTeamMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => deleteTeam(id),
    onSettled: async () => {
      await queryClient.invalidateQueries({ queryKey: teamsQueryKey });
    },
  });
}
