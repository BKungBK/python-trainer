-- Profile avatars are stored in Storage; user_status only keeps the public URL.
alter table public.user_status
  add column if not exists avatar_url text;

alter table public.user_status
  drop constraint if exists user_status_avatar_url_length;

alter table public.user_status
  add constraint user_status_avatar_url_length
  check (avatar_url is null or length(avatar_url) <= 2048);

-- The app uses its existing anonymous Supabase access model. Storage enforces
-- the file type and size while the app stores only the latest public URL.
insert into storage.buckets (id, name, public, file_size_limit, allowed_mime_types)
values (
  'avatars',
  'avatars',
  true,
  5242880,
  array['image/png', 'image/jpeg', 'image/webp', 'image/gif', 'image/svg+xml']::text[]
)
on conflict (id) do update set
  public = excluded.public,
  file_size_limit = excluded.file_size_limit,
  allowed_mime_types = excluded.allowed_mime_types;

drop policy if exists "Public avatar read" on storage.objects;
create policy "Public avatar read"
  on storage.objects for select
  using (bucket_id = 'avatars');

drop policy if exists "Anonymous avatar upload" on storage.objects;
create policy "Anonymous avatar upload"
  on storage.objects for insert to anon
  with check (bucket_id = 'avatars');

-- Realtime is already enabled for user_status in the project. Keep this
-- idempotent for linked environments that have not enabled it yet.
do $$
begin
  begin
    alter publication supabase_realtime add table public.user_status;
  exception
    when duplicate_object then null;
  end;
end
$$;
