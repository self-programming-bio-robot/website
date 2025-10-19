import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "zhdanov.dev",
  description:
    "Personal website of Nikolay Zhdanov",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        {children}
      </body>
    </html>
  );
}
