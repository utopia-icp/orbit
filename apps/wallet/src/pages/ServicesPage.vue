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

        <DataLoader
          :load="load"
          :refresh-interval-ms="2500"
          :disable-refresh="disableRefresh"
          @loaded="
            result => {
              registryApps = result.apps;
              installedServices = result.services;
              uninstalledServices = result.uninstalledServices;
            }
          "
        >
          <VRow v-if="installedServices.length">
            <VCol v-for="(service, idx) in installedServices" :key="idx" cols="12" md="4">
              <VCard>
                <VCardTitle>{{ `#${idx + 1} ${service.name}` }}</VCardTitle>
                <VCardSubtitle class="text-wrap">{{ service.description }}</VCardSubtitle>
                <VCardText>
                  <VList class="bg-transparent">
                    <VListItem class="px-0">
                      <VListItemTitle class="font-weight-bold">Service ID</VListItemTitle>
                      <VListItemSubtitle>
                        <span>
                          {{ service.id }}
                          <VBtn
                            size="x-small"
                            variant="text"
                            :icon="mdiContentCopy"
                            @click="
                              copyToClipboard({
                                textToCopy: service.id,
                                sendNotification: true,
                              })
                            "
                          />
                        </span>
                      </VListItemSubtitle>
                    </VListItem>
                    <VListItem class="px-0">
                      <VListItemTitle class="font-weight-bold">Version</VListItemTitle>
                      <VListItemSubtitle>
                        <span>v{{ service.version }}</span>
                      </VListItemSubtitle>
                    </VListItem>
                    <VListItem class="px-0">
                      <VListItemTitle class="font-weight-bold">Checksum</VListItemTitle>
                      <VListItemSubtitle>
                        <span>
                          <TextOverflow :text="service.checksum" :max-length="26" />
                          <VBtn
                            size="x-small"
                            variant="text"
                            :icon="mdiContentCopy"
                            @click="
                              copyToClipboard({
                                textToCopy: service.checksum,
                                sendNotification: true,
                              })
                            "
                          />
                        </span>
                      </VListItemSubtitle>
                    </VListItem>
                    <VListItem class="px-0">
                      <VListItemTitle class="font-weight-bold">Tags</VListItemTitle>
                      <VListItemSubtitle>
                        <span>{{ service.tags.join(', ') }}</span>
                      </VListItemSubtitle>
                    </VListItem>
                  </VList>
                </VCardText>
                <VCardActions class="mx-2">
                  <ServiceChangeAction
                    :service-id="service.id"
                    :app="service"
                    class="px-1 mb-2"
                    size="small"
                    color="default"
                    variant="text"
                    :append-icon="mdiTuneVariant"
                    @opened="disableRefresh = $event"
                  >
                    configure
                  </ServiceChangeAction>
                  <VSpacer />
                  <VBtn
                    variant="text"
                    :append-icon="mdiOpenInNew"
                    density="comfortable"
                    color="primary"
                    :href="formatServiceUrl(service.id)"
                    target="_blank"
                  >
                    open
                  </VBtn>
                </VCardActions>
              </VCard>
            </VCol>
          </VRow>

          <VRow>
            <VCol v-for="(service, idx) in uninstalledServices" :key="idx" cols="12" md="4">
              <VCard>
                <VCardTitle>{{ `#${idx + 1} Created Service` }}</VCardTitle>
                <VCardText class="py-0">
                  <VList class="bg-transparent py-0">
                    <VListItem class="px-0">
                      <VListItemTitle class="font-weight-bold">Service ID</VListItemTitle>
                      <VListItemSubtitle>
                        <span>
                          {{ service.id }}
                          <VBtn
                            size="x-small"
                            variant="text"
                            :icon="mdiContentCopy"
                            @click="
                              copyToClipboard({
                                textToCopy: service.id,
                                sendNotification: true,
                              })
                            "
                          />
                        </span>
                      </VListItemSubtitle>
                    </VListItem>
                  </VList>
                </VCardText>
                <VCardActions>
                  <VSpacer />
                  <InstallServiceAction
                    :service-id="service.id"
                    :apps="registryApps"
                    class="px-1 mb-2"
                    size="small"
                    color="default"
                    variant="text"
                    :append-icon="mdiTuneVariant"
                    @opened="disableRefresh = $event"
                  >
                    configure
                  </InstallServiceAction>
                </VCardActions>
              </VCard>
            </VCol>
            <VCol cols="12" md="4">
              <VCard :loading="creatingNewService">
                <VCardTitle>Install new service</VCardTitle>
                <VCardText>Reserve a new service to deploy your new application.</VCardText>
                <VCardActions class="mt-2">
                  <VSpacer />
                  <VBtn
                    color="primary"
                    variant="text"
                    size="small"
                    :disabled="installBtnIsDisabled"
                    @click="addService"
                  >
                    Add service
                  </VBtn>
                </VCardActions>
              </VCard>
            </VCol>
          </VRow>
        </DataLoader>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiContentCopy, mdiOpenInNew, mdiTuneVariant } from '@mdi/js';
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VCardActions, VCardSubtitle, VCardTitle, VListItem, VSpacer } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import ServiceChangeAction from '~/components/services/ServiceChangeAction.vue';
import InstallServiceAction from '~/components/services/InstallServiceAction.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Routes } from '~/configs/routes.config';
import logger from '~/core/logger.core';
import { services } from '~/plugins/services.plugin';
import { useStationStore } from '~/stores/station.store';
import type {
  PageProps,
  RegistryApp,
  ServiceInstalled,
  ServiceUninstalled,
} from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { RequestDomains } from '~/types/station.types';
import { copyToClipboard, fetchCanisterChecksum } from '~/utils/app.utils';
import { variantIs } from '~/utils/helper.utils';
import { icAgent } from '~/core/ic-agent.core';
import TextOverflow from '~/components/TextOverflow.vue';
import { useRouter } from 'vue-router';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const i18n = useI18n();
const pageTitle = computed(() => props.title || i18n.t('pages.services.title'));
const station = useStationStore();
const controlPanel = services().controlPanel;
const registryApps = ref<RegistryApp[]>([]);
const installedServices = ref<ServiceInstalled[]>([]);
const uninstalledServices = ref<ServiceUninstalled[]>([]);
const disableRefresh = ref(false);
const showInstall = ref(false);

const formatServiceUrl = (serviceId: string) =>
  import.meta.env.PROD ? `https://${serviceId}.icp0.io` : `http://${serviceId}.localhost:4943/`;

const fetchRegistryApps = async (): Promise<RegistryApp[]> => {
  const apps: RegistryApp[] = [];
  const result = await controlPanel.searchRegistry({
    filter_by: [{ Kind: { WasmModule: null } }, { Namespace: 'demo' }],
    pagination: [],
    sort_by: [],
  });

  for (const entry of result.entries) {
    const name = entry.name.split('/')[1];
    let index = apps.findIndex(app => app.name === name);

    if (index === -1) {
      apps.push({
        name,
        description: entry.description,
        tags: entry.tags,
        versions: [],
      });

      index = apps.length - 1;
    }

    if (variantIs(entry.value, 'WasmModule')) {
      const { artifact } = await controlPanel.getArtifact(entry.value.WasmModule.wasm_artifact_id);

      apps[index].versions.push({
        version: entry.value.WasmModule.version,
        checksum: artifact.hash,
        size: Number(artifact.size),
        wasm: artifact.artifact,
      });
    }
  }

  return apps.filter(app => app.versions.length > 0);
};

const load = async (): Promise<{
  services: ServiceInstalled[];
  uninstalledServices: ServiceUninstalled[];
  apps: RegistryApp[];
}> => {
  const agent = icAgent.get();
  const [managedServices, apps] = await Promise.all([
    station.service.listRequests({
      types: [{ CreateExternalCanister: null }],
      statuses: [{ Completed: null }],
    }),
    fetchRegistryApps(),
  ]);

  const services: ServiceInstalled[] = [];
  const uninstalledServices: ServiceUninstalled[] = [];
  for (const request of managedServices.requests) {
    if (
      variantIs(request.operation, 'CreateExternalCanister') &&
      request.operation.CreateExternalCanister.canister_id?.[0]
    ) {
      const checksum =
        (await fetchCanisterChecksum(
          agent,
          request.operation.CreateExternalCanister.canister_id[0],
        )) || '';
      const app = apps.find(
        app => app.versions.findIndex(version => version.checksum === checksum) !== -1,
      );

      if (app) {
        services.push({
          id: request.operation.CreateExternalCanister.canister_id[0].toText(),
          checksum,
          name: app.name,
          description: app.description,
          tags: app.tags,
          version: app.versions.find(version => version.checksum === checksum)!.version,
          updates: app.versions.filter(version => version.checksum !== checksum),
        });
      } else {
        uninstalledServices.push({
          id: request.operation.CreateExternalCanister.canister_id[0].toText(),
        });
      }
    }
  }

  return { services, uninstalledServices, apps };
};

const creatingNewService = ref(false);

const addService = async () => {
  try {
    creatingNewService.value = true;
    const request = await station.service.addService();
    useOnSuccessfulOperation(request);
  } catch (error) {
    logger.error(`Failed to submit account ${error}`);

    useOnFailedOperation();
  } finally {
    creatingNewService.value = false;
  }
};

const router = useRouter();

onMounted(async () => {
  if (router.currentRoute.value.query?.install) {
    showInstall.value = true;
  }
});

const installBtnIsDisabled = computed(() => {
  return !showInstall.value || creatingNewService.value;
});
</script>
