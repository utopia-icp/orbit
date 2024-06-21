<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs" />
    </template>
    <template #main-body>
      <PageBody>
        <AuthCheck :privileges="[Privilege.ListRequests]">
          <RecentRequests
            class="mb-4"
            :see-all-link="{
              name: Routes.Requests,
              query: { group_by: RequestDomains.System },
            }"
            :types="[{ ChangeExternalCanister: [] }, { CreateExternalCanister: null }]"
            hide-not-found
          />
        </AuthCheck>

        <VRow>
          <VCol cols="12" md="6">
            <VCard>
              <VCardTitle>Unstoppable App</VCardTitle>
              <VCardText>
                <VList lines="two" class="bg-transparent">
                  <VListItem class="px-0">
                    <VListItemTitle class="font-weight-bold">Service ID</VListItemTitle>
                    <VListItemSubtitle>
                      <span>
                        {{ serviceId }}
                        <VBtn
                          size="x-small"
                          variant="text"
                          :icon="mdiContentCopy"
                          @click="
                            copyToClipboard({
                              textToCopy: serviceId,
                              sendNotification: true,
                            })
                          "
                        />
                      </span>
                    </VListItemSubtitle>
                  </VListItem>
                </VList>
              </VCardText>
              <VCardActions class="mx-2">
                <ServiceChangeAction
                  :service-id="serviceId"
                  class="px-1 mb-2"
                  size="small"
                  color="default"
                  variant="text"
                  :append-icon="mdiTuneVariant"
                >
                  configure
                </ServiceChangeAction>
                <VSpacer />
                <VBtn
                  variant="text"
                  :append-icon="mdiOpenInNew"
                  density="comfortable"
                  color="primary"
                  :href="serviceUrl"
                  target="_blank"
                >
                  open
                </VBtn>
              </VCardActions>
            </VCard>
          </VCol>
        </VRow>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiContentCopy, mdiOpenInNew, mdiTuneVariant } from '@mdi/js';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VCardActions, VSpacer } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import PageLayout from '~/components/PageLayout.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import ServiceChangeAction from '~/components/services/ServiceChangeAction.vue';
import ServicesSettingsAction from '~/components/services/ServicesSettingsAction.vue';
import { Routes } from '~/configs/routes.config';
import { useStationStore } from '~/stores/station.store';
import type { PageProps } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { RequestDomains } from '~/types/station.types';
import { copyToClipboard } from '~/utils/app.utils';
import { variantIs } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const i18n = useI18n();
const pageTitle = computed(() => props.title || i18n.t('pages.services.title'));
const serviceId = '2v7xv-qiaaa-aaaaa-aaaea-cai';
const serviceUrl = `https://${serviceId}.icp0.io`;
const station = useStationStore();

const load = async () => {
  const [result] = await Promise.all([
    station.service.listRequests({
      types: [{ CreateExternalCanister: null }],
      statuses: [{ Completed: null }],
    }),
  ]);

  let count = 1;

  const services: Array<{ name: string; id: Principal }> = [];
  for (const request of result.requests) {
    if (
      variantIs(request.operation, 'CreateExternalCanister') &&
      request.operation.CreateExternalCanister.canister_id?.[0]
    ) {
      services.push({
        id: request.operation.CreateExternalCanister.canister_id[0],
        name: `#${count} Unstoppable App`,
      });

      count++;
    }
  }

  return services;
};
</script>
