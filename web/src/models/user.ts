import { stringKey } from '@/models/generic';

export interface UserInterface extends stringKey<number | string | null> {
  id: number;
  first_name: string;
  last_name: string;
  email: string;
  phone: string | null;
  avatar: string | null;
  role: string;
  created_at: string;
  updated_at: string;
}

export interface EditableUserInterface extends UserInterface {
  current_password: string|null;
  new_password: string|null;
  country_code: string|null;
}
