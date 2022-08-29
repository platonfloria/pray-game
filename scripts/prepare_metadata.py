import argparse
import base64
import os
import json
import shutil

from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
from cryptography.hazmat.primitives.ciphers.aead import AESGCM


def sha256_hash(data):
    digest = hashes.Hash(hashes.SHA256())
    digest.update(data)
    return base64.b64encode(digest.finalize()).decode()


def get_key(passphrase):
    salt = os.urandom(16)
    kdf = PBKDF2HMAC(
        algorithm=hashes.SHA256(),
        length=32,
        salt=salt,
        iterations=1,
    )
    key = kdf.derive(passphrase.encode())
    return salt, key


def encrypt(key, plaintext, associated_data):
    aesgcm = AESGCM(key)
    nonce = os.urandom(12)
    cyphertext = aesgcm.encrypt(nonce, plaintext, associated_data)
    return nonce, cyphertext


def main(collection_dir, metadata_cid, batch_size):
    password = 'password'
    salt, key = get_key(password)

    output_dir = f'output/{metadata_cid}'
    if os.path.isdir(output_dir):
        shutil.rmtree(output_dir)
    os.mkdir(output_dir)
    token_ids = sorted(os.listdir(f'{collection_dir}/json'))
    batch_number = 0
    data = {}
    for token_id in token_ids:
        j = int(token_id) % 9  # TEMP
        with open(f'{collection_dir}/json/{j}', 'rb') as metadata_file, \
            open(f'{collection_dir}/images/{j}.png', 'rb') as media_file:
            contents = metadata_file.read()
            metadata = json.loads(contents)
            data[token_id] = {
                'title': metadata['name'],
                'description': metadata['description'],
                'media': metadata['image'].replace('ipfs:/', 'https://ipfs.io/ipfs'),
                'media_hash': sha256_hash(media_file.read()),
                'reference': f'ipfs://{metadata_cid}/{j}',
                'reference_hash': sha256_hash(contents)
            }
        
        if batch_size is not None and len(data) == batch_size:
            plaintext = json.dumps(data)
            nonce, cyphertext = encrypt(key, plaintext.encode(), None)
            with open(f'{output_dir}/{batch_number}', 'w') as f:
                f.write(base64.b64encode(b''.join([salt, nonce, cyphertext])).decode())
            batch_number += 1
            data.clear()

    
    if batch_size is None or data != {}:
        plaintext = json.dumps(data)
        nonce, cyphertext = encrypt(key, plaintext.encode(), None)
        with open(f'{output_dir}/{batch_number}', 'w') as f:
            f.write(base64.b64encode(b''.join([salt, nonce, cyphertext])).decode())

    # plaintext = 'this is some random text'
    # nonce, cyphertext = encrypt(key, plaintext.encode(), None)
    # return base64.b64encode(b''.join([salt, nonce, cyphertext])).decode()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Encrypt metadata of the colleciton')
    parser.add_argument('--dir', type=str, required=True, help='Directory of the collection')
    parser.add_argument('--cid', type=str, required=True, help="Collection's metadata CID")
    parser.add_argument('--batch-size', type=int, help="Number of tokens per metadata batch")

    args = parser.parse_args()

    main(args.dir, args.cid, args.batch_size)
