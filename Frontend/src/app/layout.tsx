import React from 'react';
import './globals.css';

export const metadata = {
  title: "Stellara AI",
  description: "Learn. Trade. Connect. Powered by AI on Stellar.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="min-h-screen bg-[#f9fafb] text-[#111827]">
        {children}
      </body>
    </html>
  );
}
