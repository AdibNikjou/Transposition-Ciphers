from math import ceil

class cipher:    
    
    @staticmethod
    def encrypt_caesar(plainText,shift) -> str:
        result = ""
        for char in plainText:
            if char.isalpha():
                if char.isupper():
                    shifted_char = chr((ord(char) - ord('A') + shift) % 26 + ord('A'))
                else:
                    shifted_char = chr((ord(char) - ord('a') + shift) % 26 + ord('a'))
                result += shifted_char
            else:
                result += char
        return result
                
    @staticmethod
    def decrypt_caesar(encryptedText: str, shift: str) -> str:
        return cipher.encrypt_caesar(encryptedText, -shift)
    
    @staticmethod
    def encrypt_railFence(plainText: str, rails: int = 2) -> str:
        trails = {i: [] for i in range(rails)}
        curr_trail = 0
        for letter in plainText:
            trails[curr_trail].append(letter)
            curr_trail = (curr_trail + 1) % rails
            
        return ''.join([char for trail in trails.values() for char in trail])
    
    @staticmethod
    def decrypt_railFence(encryptedText: str, rails: int = 2) -> str:
        LENGTH = len(encryptedText)
        trail_length = LENGTH // rails
        if LENGTH % rails != 0:
            trail_length += 1
        
        trails = [encryptedText[i:i + trail_length] for i in range(0, LENGTH, trail_length)]
        
        message_letter_list = []
        for index in range(trail_length):
            for trail in trails:
                if index < len(trail):
                    message_letter_list.append(trail[index])
        return ''.join(message_letter_list)
    
    @staticmethod
    def encrypt_coloumnar(plainText: str, key: str) -> str:
        len_key = len(key)
        len_plain = len(plainText)
        row = int(ceil(len_plain / len_key))
        matrix = [ ['Adib']*len_key for i in range(row) ]
        
        t = 0
        for r in range(row):
            for c,ch in enumerate(plainText[t : t+ len_key]):
                matrix[r][c] = ch
            t += len_key
            
        sort_order = sorted([(ch,i) for i,ch in enumerate(key)])

        cipher_text = ''
        for ch,c in sort_order:
            for r in range(row):
                cipher_text += matrix[r][c]
        return cipher_text
            
    @staticmethod
    def decrypt_coloumnar(encryptedText: str, key: str) -> str:
        len_key = len(key)
        len_plain = len(encryptedText)
        row = int(ceil(len_plain / len_key))
        
        matrix = [ ['Adib']*len_key for i in range(row) ]
        key_order = [ key.index(ch) for ch in sorted(list(key))]

        t = 0
        for c in key_order:
            for r,ch in enumerate(encryptedText[t : t+ row]):
                matrix[r][c] = ch
            t += row

        p_text = ''
        for r in range(row):
            for c in range(len_key):
                p_text += matrix[r][c] if matrix[r][c] != 'Adib' else ''
        return p_text

#-------------------------------------------------------------
# PlainText

plainText = "Salam, Man Adib Nikjou hastam."
print("plainText: " , plainText)

#-------------------------------------------------------------
# Caesar:
encrypted_caesar = cipher.encrypt_caesar(plainText,
                            shift=18
                            )
print("encrypted Caesar: ", encrypted_caesar)


decrypted_caesar = cipher.decrypt_caesar(encrypted_caesar,
                            shift=18
                            )
print("decrypted Caesar: ",decrypted_caesar)

#------------------------------------------------------------
# Rail Fence
encrypted_railFence = cipher.encrypt_railFence(plainText,
                                rails=5
                                )
print('encrypted Rail Fence: ',encrypted_railFence)


decrypted_railFence = cipher.decrypt_railFence(encrypted_railFence,
                                rails=5
                                )

print('decrypted Rail Fence: ',decrypted_railFence)

#-----------------------------------------------------------
# Coloumnar
encrypted_coloumnar = cipher.encrypt_coloumnar(plainText,
                                key="13425"
                                )
print("encrypted coloumnar: ",encrypted_coloumnar)

decrypted_coloumnar = cipher.decrypt_coloumnar(encrypted_coloumnar,
                                key="13425"
                                )
print("decrypted coloumnar: ",decrypted_coloumnar)