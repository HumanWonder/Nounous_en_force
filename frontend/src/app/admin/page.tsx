"use client";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "../hooks/useAuth";

// Déclare un type pour les données des utilisateurs (données simplifiées)
type User = {
    id: string;
    email: string;
    role: string;
    is_profile_validated: boolean;
};

export default function AdminDashboard() {
    const router = useRouter();
    const { token, isAuthenticated } = useAuth();
    const [users, setUsers] = useState<User[]>([]);
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchUsers = async () => {
            // Vérifier si le token est disponible et si l'utilisateur est authentifié
            if (!token || !isAuthenticated) {
                setMessage("Token inexistant, connexion non authentifiée");
                return;
            }

            try {
                // Requête à l'API pour obtenir les utilisateurs non validés
                const response = await fetch("http://127.0.0.1:8080/admin", {
                    method: "GET",
                    headers: {
                        "Authorization": `Bearer ${token}`, // Ajouter le token dans l'en-tête
                    },
                    credentials: "include", // Assurer la gestion des cookies pour les sessions
                });

                const data = await response.json();
                console.log(data);
                if (response.ok) {
                    setUsers(data);
                } else {
                    setMessage(data.message || "Erreur de récupération des données. Veuillez vérifier vos permissions.");
                }
            } catch (error) {
                console.error("Erreur lors de la récupération des utilisateurs:", error);
                setMessage("Erreur réseau. Vérifiez votre connexion.");
            } finally {
                setLoading(false);
            }
        };

        if (token && isAuthenticated) {
            fetchUsers();
        } else {
            setLoading(false);
        }
    }, [isAuthenticated, token, router]);

    if (loading) {
        return <p>Chargement des données...</p>;
    }

    //Affiche seulement message d'erreur si pas admin
    return message ? (
        <div>{message}</div>
    ) : (
        <div>
            <h2>Dashboard Administrateur</h2>
            <p>Liste des utilisateurs en attente de validation :</p>

            {users.length === 0 ? (
                <p>Aucun utilisateur en attente de validation.</p>
            ) : (
                <div>
                    <table className="min-w-full table-auto">
                        <thead>
                            <tr>
                                <th className="px-4 py-2 border">Nom</th>
                                <th className="px-4 py-2 border">Rôle</th>
                                <th className="px-4 py-2 border">Statut</th>
                                <th className="px-4 py-2 border">Actions</th>
                            </tr>
                        </thead>
                        <tbody>
                            {users.map(user => (
                                <tr key={user.id}>
                                    <td className="px-4 py-2 border">{user[1]}</td>
                                    <td className="px-4 py-2 border">{user[2]}</td>
                                    <td className="px-4 py-2 border">
                                        {user.is_profile_validated ? "Validé" : "En attente"}
                                    </td>
                                    <td className="px-4 py-2 border">
                                        {!user.is_profile_validated && (
                                            <button
                                                className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
                                                onClick={() => handleValidate(user.id)}
                                            >
                                                Valider
                                            </button>
                                        )}
                                    </td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            )}
        </div>
    );

    // Fonction de validation du profil
    const handleValidate = async (userId: string) => {
        try {
            const response = await fetch(`http://127.0.0.1:8080/admin/validate/${userId}`, {
                method: "POST",
                headers: {
                    "Authorization": `Bearer ${token}`,
                    "Content-Type": "application/json",
                },
                credentials: "include",
            });

            const data = await response.json();
            if (response.ok) {
                setMessage("Profil validé avec succès");
                // Recharger les utilisateurs après validation
                setUsers(prevUsers => prevUsers.filter(user => user.id !== userId));
            } else {
                setMessage(data.message || "Erreur lors de la validation du profil");
            }
        } catch (error) {
            console.error("Erreur lors de la validation du profil:", error);
            setMessage("Erreur réseau. Impossible de valider ce profil.");
        }
    };
}
