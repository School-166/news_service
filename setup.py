import os
import io

PATH_TO_DB = ""

DEFFAULT_DB_PORT = "5432"

def get_username() -> str:
    username : str = input("Please enter your username for database or press enter to get your current username: ")
    if len(username) != 0:
        return username
    return os.getenv("USERNAME")

def write_to_dotenv_file(path_to_db: str, port: str):
    file = io.FileIO(".env", "a")
    file.write(f"{PATH_TO_DB} = {path_to_db}\n
              PORT = {port}")
    file.close 

def ask_password()->str:
    password = input("Please enter password for your user: ")
    if len(password) == 0:
        print("Password must be set")
        return ask_password()
    return password

def ask_db_name()->str:
    db_name = input("Please enter name of database: ")
    if len(db_name) == 0:
        return ask_db_name()
    return db_name

def ask_db_port()->str:
    db_bort = input(f"Please enter db port({DEFFAULT_DB_PORT}): ")
    if len(port) == 4:
        return port
    return DEFFAULT_DB_PORT

def ask_host()->str:
    host = input("Please enter database host(localhost default): ")
    if len(host)==0:
        return "localhost"
    return host  

def build_path_to_db(password: str, host: str, db_name: str, db_port: str)->str:
    return f"postgresql://{get_username()}:{password}@{host}:{db_port}/{db_name}"

def main():
    print("\t\t\tHello and welcome\n")
    user = get_username()
    password = ask_password()
    host = ask_host()
    db_name = ask_db_name()
    db_port = ask_db_port()
    

    write_to_dotenv_file(build_path_to_db(password, host, db_name, db_port), port=)
    
main()
