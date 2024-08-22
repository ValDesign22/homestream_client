import { createRouter, createWebHistory } from 'vue-router';

const router = createRouter({
  history: createWebHistory(),
  scrollBehavior() {
    return {
      top: 0,
      behavior: 'smooth'
    };
  },
  routes: [
    {
      path: '/',
      component: () => import('@/views/HomeView.vue')
    }, {
      path: '/register',
      component: () => import('@/views/RegisterView.vue')
    }, {
      path: '/browse',
      component: () => import('@/views/BrowseView.vue')
    }, {
      path: '/details/:id',
      component: () => import('@/views/DetailsView.vue')
    }, {
      path: '/watch/:id',
      component: () => import('@/views/WatchView.vue')
    }, {
      path: '/search',
      component: () => import('@/views/SearchView.vue')
    }, {
      path: '/profiles',
      component: () => import('@/views/ProfilesView.vue')
    }, {
      path: '/settings',
      component: () => import('@/views/SettingsView.vue')
    }
  ]
});

export default router;
