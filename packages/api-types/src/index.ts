// Exemple de type partagé pour l'utilisateur
export interface User {
  id: string;
  email: string;
  name: string | null;
}

// Exemple pour la réponse d'authentification
export interface AuthResponse {
  token: string;
  user: User;
}
