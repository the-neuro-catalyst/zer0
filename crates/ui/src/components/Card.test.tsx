import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import '@testing-library/jest-dom/vitest';
import { Card, InfoCard, ActionCard } from './Card';

describe('Card Components', () => {
  it('should render Card with children', () => {
    render(<Card>Hello Card</Card>);
    expect(screen.getByText('Hello Card')).toBeDefined();
  });

  it('should render InfoCard with children', () => {
    render(<InfoCard>Hello InfoCard</InfoCard>);
    expect(screen.getByText('Hello InfoCard')).toBeDefined();
  });

  it('should render ActionCard with children', () => {
    render(<ActionCard>Hello ActionCard</ActionCard>);
    expect(screen.getByText('Hello ActionCard')).toBeDefined();
  });
});
