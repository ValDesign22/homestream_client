export default {
  languages: {
    en: 'English',
    fr: 'French',
  },
  app: {
    name: 'HomeStream',
    footerVersion: 'HomeStream v{version}',
  },
  components: {
    navbar: {
      search: 'Search...',
      profile: 'Profile',
      switchProfile: 'Switch profile',
      settings: 'Settings',
    },
    tree: {
      selectFolder: 'Select a folder',
    },
  },
  pages: {
    browse: {
      watch: 'Watch',
      details: 'Details',
      continue: 'Continue watching',
      watchlist: 'Watchlist',
      favorites: 'Favorites',
    },
    details: {
      watch: 'Watch',
      runtime: '{runtime} minutes',
      watchlist: {
        add: 'Add to watchlist',
        remove: 'Remove from watchlist',
      },
      favorites: {
        add: 'Add to favorites',
        remove: 'Remove from favorites',
      },
      season: 'Season {season}',
      episode: 'Episode {episode}',
      collection: 'Collection',
      recommendations: 'Recommendations',
    },
    home: {
      selectProfile: 'Select a profile',
      noProfiles: 'No profiles found',
      manageProfiles: 'Manage profiles',
    },
    profiles: {
      title: 'Profiles',
      noProfiles: 'No profiles found',
      back: 'Back',
      addProfile: 'Add profile',
      dialog: {
        title: 'Add profile',
        description: 'Add new profile to HomeStream',
        name: 'Name',
        password: 'Password',
        role: 'Role',
        cancel: 'Cancel',
        add: 'Add',
      },
    },
    search: {
      noResults: 'No results found for "{query}"',
      results: 'Results for "{query}":',
    },
    register: {
      steps: {
        connect: 'Connect',
        connectDescription: 'Connect to your HomeStream server',
        language: 'Language',
        languageDescription: 'Select your preferred language',
        folders: 'Folders',
        foldersDescription: 'Select the folders you want to watch',
        success: 'Success',
        successDescription: 'You are ready to start watching!',
      },
      form: {
        address: 'HomeStream server address',
        language: 'Preferred language',
        noFolders: 'No folders have been added yet',
        selectType: 'Select a type',
        mediaType: 'Media type',
        movies: 'Movies',
        tvShows: 'TV Shows',
        folderName: 'Folder name',
        selectFolder: 'Select a folder',
        addFolder: 'Add folder',
        success: 'Your configuration has been saved successfully! You can now start watching your favorite movies and TV shows.',
        back: 'Back',
        skip: 'Skip',
        next: 'Next',
        submit: 'Submit',
      },
    },
    watch: {
      subtitles: 'Subtitles',
      season: 'Season {season}',
    },
  },
  settings: {
    title: 'Settings',
    tabs: {
      general: 'General',
      appearance: 'Appearance',
      server: 'Server',
    },
    general: {
      title: 'General',
    },
    appearance: {
      title: 'Appearance',
      language: 'Language:',
      selectLanguage: 'Select a language',
    },
    server: {
      title: 'Server',
      serverVersion: 'Server version:',
      update: 'Update',
    },
  }
};