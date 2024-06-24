<template>
  <div class="d-flex flex-column ga-0 text-caption">Allocate service resources.</div>
</template>

<script setup lang="ts">
import { Request } from '~/generated/station/station.did';

const _ = withDefaults(
  defineProps<{
    request: Request;
    operation: unknown;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);
</script>
