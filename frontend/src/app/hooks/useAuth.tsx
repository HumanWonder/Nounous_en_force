"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
// import Cookies from "js-cookie";
const isDev = process.env.NEXT_PUBLIC_ENV === "development";

function isTokenExipred(token: string | null): boolean {
    if (!token) return true;

    try {
        const payload = JSON.parse(atob(token.split(".")[1])); // Décodage du JWT
        const exp = payload.exp * 1000;
        return Date.now() > exp;
    } catch (error) {
        console.error("Erreur lors du décodage du token :", error);
        return true;
    }
}

export function useAuth() {
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [token, setToken] = useState<string | null>(null); // état pour le token
    const router = useRouter();

    useEffect(() => {
        console.log("getting token..... isDev is : ", isDev);
        if (isDev) {
            const storedToken = localStorage.getItem("auth_token");
            if (storedToken && !isTokenExipred(storedToken)) {
                setToken(storedToken);
                console.log("TOKEN FOUND");
                setIsAuthenticated(true); // Met à jour l'état selon la présence du token
            } else {
                console.log("No token, or token expired");
                setToken(null);
                setIsAuthenticated(false);
                localStorage.removeItem("auth_token"); //Nettoyage si token expiré
            }
        }
    }, []);
    // const token = Cookies.get("auth_token"); // Récupère le token depuis les cookies => à la place de localStorage

    const logout = async () => {

        try {
            //Appel API /logout
            const response = await fetch("http://127.0.0.1:8080/logout", {
                method: "POST",
                credentials: "include", //Pour s'assurer que le cookie est bien envoyé
            });

            if (response.ok && isDev) {
                // Cookies.remove("auth_token"); // Supprime le token côté client
                localStorage.removeItem("auth_token");

                setIsAuthenticated(false); // Met à jour l'état
                setToken(null) //Réinitialise le token
                router.push("/login");  //Redirection vers la page de connexion
            }
        } catch (error) {
            console.error("Erreur lors de la déconnexion : ", error);
        }
    };

    return { isAuthenticated, token, logout };
}
