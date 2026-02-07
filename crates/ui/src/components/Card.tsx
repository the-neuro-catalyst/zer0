import React from 'react';

export const Card = ({ children }: { children: React.ReactNode }) => (
  <div className="rounded-lg border bg-card text-card-foreground shadow-sm">{children}</div>
);

export const InfoCard = ({ children }: { children: React.ReactNode }) => (
  <div className="rounded-lg border bg-card text-card-foreground shadow-sm w-full h-full text-center flex justify-center items-center">
    {children}
  </div>
);

export const ActionCard = ({ children }: { children: React.ReactNode }) => (
  <div className="rounded-lg border bg-card text-card-foreground shadow-sm w-full h-full text-center flex justify-center items-center hover:bg-muted/50 transition-colors">
    {children}
  </div>
);
