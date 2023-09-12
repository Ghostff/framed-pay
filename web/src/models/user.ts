import { stringKey } from '@/models/generic';

export interface UserInterface extends stringKey<number | string | null> {
  id: number;
  first_name: string;
  last_name: string;
  photo: string | null;
  email: string;
  role: string;
  created_at: string;
  updated_at: string;
}
