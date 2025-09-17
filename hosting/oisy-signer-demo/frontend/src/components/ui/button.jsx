import { cva } from 'class-variance-authority';
import { cn } from '@/libs/utils';
import React from 'react';

const buttonVariants = cva(
  'inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none',
  {
    variants: {
      variant: {
        default: 'bg-blue-600 text-white hover:bg-blue-700',
        destructive: 'bg-red-600 text-white hover:bg-red-700',
        connect: 'bg-blue-600 text-white hover:bg-blue-700',
        disconnect: 'bg-black text-white hover:bg-gray-800',
        'disconnect-dark': 'bg-red-600 text-white hover:bg-red-700',
      },
      size: {
        default: 'h-10 px-4 py-2',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  }
);

export const Button = React.forwardRef(({ className, variant, size, ...props }, ref) => {
  return (
    <button className={cn(buttonVariants({ variant, size }), className)} ref={ref} {...props}>
      {props.children}
    </button>
  );
});
Button.displayName = 'Button';
