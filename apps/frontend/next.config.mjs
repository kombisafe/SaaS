/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  // Permet d'utiliser les types partagés du monorepo
  transpilePackages: ['@saas/api-types'],
};

export default nextConfig;
