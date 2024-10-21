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
    }
  ]
});

router.afterEach((to, from) => {
  if (from.path === '/' && to.path === '/browse') {
    to.meta.transition = 'center';
  }
  if (from.path === '/browse' && to.path === '/') {
    to.meta.transition = 'center';
  }
  if (from.path === '/browse' && to.path === '/details') {
    to.meta.transition = 'center';
  }
  if ((from.path === '/browse' || from.path === '/details') && to.path === '/watch') {
    to.meta.transition = 'scale';
  }
});

export default router;
