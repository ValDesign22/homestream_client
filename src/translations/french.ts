export default {
  languages: {
    en: 'Anglais',
    fr: 'Français',
  },
  themes: {
    slate: 'Ardoise',
    zinc: 'Zinc',
  },
  app: {
    name: 'HomeStream',
    footerVersion: 'HomeStream v{version}',
  },
  components: {
    navbar: {
      search: 'Rechercher...',
      profile: 'Profile',
      switchProfile: 'Changer de profile',
      settings: 'Paramètres',
    },
    tree: {
      selectFolder: 'Sélectionner un dossier',
    },
  },
  pages: {
    browse: {
      watch: 'Regarder',
      details: 'Détails',
      continue: 'Continuer à regarder',
      watchlist: 'Liste de lecture',
      favorites: 'Favoris',
    },
    details: {
      watch: 'Regarder',
      continue: 'Continuer à regarder',
      runtime: '{runtime} minutes',
      time_left: 'Il reste {time} minutes',
      watchlist: {
        add: 'Ajouter à la liste de lecture',
        remove: 'Retirer de la liste de lecture',
      },
      favorites: {
        add: 'Ajouter aux favoris',
        remove: 'Retirer des favoris',
      },
      season: 'Saison {season}',
      episode: 'Épisode {episode}',
      collection: 'Collection',
      recommendations: 'Recommandations',
    },
    home: {
      selectProfile: 'Sélectionner un profile',
      noProfiles: 'Aucun profile trouvé',
      manageProfiles: 'Gérer les profiles',
    },
    profiles: {
      title: 'Profiles',
      noProfiles: 'Aucun profile trouvé',
      back: 'Retour',
      addProfile: 'Ajouter un profile',
      dialog: {
        title: 'Ajouter un profile',
        description: 'Ajouter un nouveau profile à HomeStream',
        name: 'Nom',
        password: 'Mot de passe',
        role: 'Rôle',
        cancel: 'Annuler',
        add: 'Ajouter',
      },
    },
    search: {
      noResults: 'Aucun résultat trouvé pour "{query}"',
      results: 'Résultats pour "{query}" :',
    },
    watch: {
      subtitles: 'Sous-titres',
      season: 'Saison {season}',
      episode: 'Saison {season}, Épisode {episode} : {title}',
      next: 'Épisode suivant',
      track_none: 'Aucun',
    },
    offline: {
      title: 'Hors ligne',
      description: 'Il semble que vous êtes hors ligne. Veuillez vérifier votre connexion Internet et réessayer.',
    }
  },
  settings: {
    title: 'Paramètres',
    tabs: {
      general: 'Général',
      appearance: 'Apparence',
      server: 'Serveur',
    },
    general: {
      title: 'Général',
      version: 'Version :',
      update: 'Mettre à jour',
      restart: 'Redémarrer',
    },
    appearance: {
      title: 'Apparence',
      language: 'Langue :',
      selectLanguage: 'Sélectionner une langue',
      theme: 'Thème :',
      selectTheme: 'Sélectionner un thème',
    },
    server: {
      title: 'Serveur',
      serverVersion: 'Version du serveur:',
      update: 'Mettre à jour',
      noConnection: 'Pas de connexion au serveur',
    },
  },
  register: {
    steps: {
      connect: 'Connecter',
      connectDescription: 'Connectez-vous à votre serveur HomeStream',
      language: 'Langue',
      languageDescription: 'Sélectionnez votre langue préférée',
      folders: 'Dossiers',
      foldersDescription: 'Sélectionnez les dossiers que vous souhaitez regarder',
      success: 'Succès',
      successDescription: 'Vous êtes prêt à commencer à regarder!',
    },
    form: {
      address: 'Adresse du serveur HomeStream',
      language: 'Langue préférée',
      noFolders: 'Aucun dossier n\'a été ajouté pour le moment',
      selectType: 'Sélectionner un type',
      mediaType: 'Type de média',
      movies: 'Films',
      tvShows: 'Séries TV',
      folderName: 'Nom du dossier',
      selectFolder: 'Sélectionner un dossier',
      addFolder: 'Ajouter un dossier',
      success: 'Votre configuration a été enregistrée avec succès! Vous pouvez maintenant commencer à regarder vos films et séries TV préférés.',
      back: 'Retour',
      skip: 'Passer',
      next: 'Suivant',
      submit: 'Soumettre',
    },
  },
};