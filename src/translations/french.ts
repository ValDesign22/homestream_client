import { MessageFormat } from '.';

export const french: MessageFormat = {
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
      runtime: '{runtime} minutes',
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
    watch: {
      subtitles: 'Sous-titres',
      season: 'Saison {season}',
    },
  },
  settings: {
    title: 'Paramètres',
    tabs: {
      general: 'Général',
      server: 'Serveur',
    },
    general: {
      title: 'Général',
    },
    server: {
      title: 'Serveur',
      serverVersion: 'Version du serveur:',
      update: 'Mettre à jour',
    },
  }
};