// Ce layout vérifiera si l'utilisateur est connecté
// et protégera les pages du dashboard.

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  // TODO: Ajouter la logique d'authentification ici
  // const isAuthenticated = false;
  // if (!isAuthenticated) redirect('/login');

  return <section>{children}</section>;
}
