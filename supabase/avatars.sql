-- Avatar storage setup for Python Trainer.
-- Run this once in the Supabase SQL editor for the same project used by the app.
-- The app has no Supabase Auth session, so avatar objects must be publicly
-- readable for presence cards to work in other users' desktop clients.

insert into storage.buckets (id, name, public)
values ('avatars', 'avatars', true)
on conflict (id) do update set public = true;

drop policy if exists "avatars are publicly readable" on storage.objects;
create policy "avatars are publicly readable"
on storage.objects for select
to public
using (bucket_id = 'avatars');

-- Uploads are made with the project's anon key by the desktop app.
drop policy if exists "avatars can be uploaded by the app" on storage.objects;
create policy "avatars can be uploaded by the app"
on storage.objects for insert
to anon, authenticated
with check (
  bucket_id = 'avatars'
  and lower(coalesce(metadata->>'mimetype', '')) in (
    'image/jpeg', 'image/png', 'image/webp', 'image/gif', 'image/svg+xml'
  )
  and coalesce((metadata->>'size')::bigint, 0) <= 5242880
);

drop policy if exists "avatars can be replaced by the app" on storage.objects;
create policy "avatars can be replaced by the app"
on storage.objects for update
to anon, authenticated
using (bucket_id = 'avatars')
with check (
  bucket_id = 'avatars'
  and lower(coalesce(metadata->>'mimetype', '')) in (
    'image/jpeg', 'image/png', 'image/webp', 'image/gif', 'image/svg+xml'
  )
  and coalesce((metadata->>'size')::bigint, 0) <= 5242880
);

drop policy if exists "avatars can be cleaned up by the app" on storage.objects;
create policy "avatars can be cleaned up by the app"
on storage.objects for delete
to anon, authenticated
using (bucket_id = 'avatars');
