"use client";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "../../../hooks/useAuth";
import type { OwnerFullProfile } from "../../../types/user";

import { StepProgressBar } from "@/components/form/StepProgressBar";
import { StepControls } from "@/components/form/StepControls";


const steps = [
    "Informations sur la crèche",
    "Description pédagogique",
    "Fiche(s) référente(s) pour contacter la crèche",
    "Annonce(s) et besoin(s)"
];

export default function NurseryRegister() {
    const router = useRouter();
    const { token, isAuthenticated } = useAuth();

    const [currentStep, setCurrentStep] = useState(0);

    const [formData, setFormData] = useState<OwnerFullProfile>({
        nursery: [{ name: "", address: "", phone: "", email: "", website: "", type: "" }],
        description: [{ pedagogy: "", specificities: "", philosophy: "" }],
        responsibles: [{ first_name: "", last_name: "", phone: "", email: "", role: "" }],
        needs: [{ searched_position: "", replacement_reason: "", estimated_duration: "", availability_periods: "", hours_per_week: "", main_tasks: "", required_skills: "", suggested_salary: "" }],
    },);

    // Charge les données du formulaire si existantes dans localStorage
    useEffect(() => {
        if (typeof window !== "undefined") {
            const storedFormData = localStorage.getItem("formData");
            if (storedFormData) {
                setFormData(JSON.parse(storedFormData));
            }
        }
    }, []);

    //Sauvegarde des données à chaque changement
    useEffect(() => {
        if (typeof window !== "undefined") {
            localStorage.setItem("formData", JSON.stringify(formData));
        }
    }, [formData]);

    const handleNext = () => {
        if (currentStep < steps.length - 1) {
            setCurrentStep(currentStep + 1);
        }
    };

    const handleBack = () => {
        if (currentStep > 0) {
            setCurrentStep(currentStep - 1);
        }
    };

    const handleChange = (section: keyof OwnerFullProfile, field: any, index?: number) => {
        setFormData((prev) => {
            const updated = { ...prev };
            if (Array.isArray(updated[section]) && typeof index === "number") {
                updated[section][index] = { ...updated[section][index], ...field };
            } else {
                updated[section] = { ...updated[section], ...field };
            }
            return updated;
        });
    };

    //Envoi du formulaire au back dès grâce au clic du bouton "Valider" (bouton type = submit)
    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!token || !isAuthenticated) {
            alert("Token inexistant, connexion non authentifiée, redirection vers login");
            return;
        }

        console.log("Form data before sending to server : \n", formData);

        //Data formattée pour correspondre exactement à la struct attendue dans le back-end
        const preparedData = {
            nursery: formData.nursery.map(item => ({
                name: String(item.name),
                address: String(item.address),
                phone: String(item.phone),
                email: String(item.email),
                website: String(item.website),
                type: String(item.type),
            })),
            needs: formData.needs.map(item => ({
                searched_position: String(item.searched_position),
                replacement_reason: String(item.replacement_reason),
                estimated_duration: String(item.estimated_duration),
                availability_periods: String(item.availability_periods),
                hours_per_week: String(item.hours_per_week),
                main_tasks: String(item.main_tasks),
                required_skills: String(item.required_skills),
                suggested_salary: String(item.suggested_salary),
            })),
            responsibles: formData.responsibles.map(item => ({
                first_name: String(item.first_name),
                last_name: String(item.last_name),
                phone: String(item.phone),
                email: String(item.email),
                role: String(item.role),
            })),
            description: formData.description.map(item => ({
                pedagogy: String(item.pedagogy),
                specificities: String(item.specificities),
                philosophy: String(item.philosophy),
            })), 
        };


        console.log("Prepared Data ready to be sent to server : \n", preparedData);
        try {

            const response = await fetch("http://127.0.0.1:8080/register/owner", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Bearer ${token}`,
                },
                body: JSON.stringify(preparedData),
            });
            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(errorText);
            }
            // setTimeout(()=> router.push("/profile"), 2000);
            localStorage.removeItem("formData");
        } catch (error) {
            console.error("Erreur lors de l'envoi du formulaire :", error);
        }
    };

    return (
        <form>
            <div className="max-w-2xl mx-auto p-4">

                <StepProgressBar steps={steps} currentStep={currentStep} />

                {/* Ici on affiche un composant différent selon currentStep */}
                {/* Informations personnelles */}
                {currentStep === 0 && (
                    <div className="space-y-2">
                        <label className="block">
                            <input
                                type="text"
                                value={formData.nursery[0].name}
                                onChange={(e) => handleChange("nursery", [{ ...formData.nursery[0], name: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Nom de la crèche"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.nursery[0].address}
                                onChange={(e) => handleChange("nursery", [{ ...formData.nursery[0], address: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Adresse complète"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="tel"
                                value={formData.nursery[0].phone}
                                onChange={(e) => handleChange("nursery", [{ ...formData.nursery[0], phone: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Numéro de téléphone de la crèche (ou du référant principal)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="email"
                                value={formData.nursery[0].email}
                                onChange={(e) => handleChange("nursery", [{ ...formData.nursery[0], email: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Adresse e-mail de la crèche (ou du référant principal)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="tel"
                                value={formData.nursery[0].website}
                                onChange={(e) => handleChange("nursery", [{ ...formData.nursery[0], website: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Site internet si existant"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                onChange={(e) => handleChange("nursery", [{ ...formData.nursery[0], type: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Type de structure (ex: crèche parentale, crèche associative...)"

                            />
                        </label>

                    </div>
                )}
                {currentStep === 1 && (
                    <div className="space-y-2">
                        <label className="block">
                            <textarea
                                value={formData.description[0].pedagogy}
                                onChange={(e) => handleChange("description", [{ ...formData.description[0], pedagogy: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Approche pédagogique (ex: Montessori, Reggio, Pikler...)"
                            />
                        </label>
                        <label className="block">
                            <textarea
                                value={formData.description[0].specificities}
                                onChange={(e) => handleChange("description", [{ ...formData.description[0], specificities: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Spécificités (ex: horaires atypiques, accueil d'enfants à besoins spécifiques...)"
                            />
                        </label>
                        <label className="block">
                            <textarea
                                value={formData.description[0].philosophy}
                                onChange={(e) => handleChange("description", [{ ...formData.description[0], philosophy: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Philosophie, valeurs ou cadre éducatif"
                            />
                        </label>
                    </div>
                )}
                {currentStep === 2 && (
                    <div className="space-y-2">
                        <label className="block">
                            <input
                                type="text"
                                value={formData.responsibles[0].first_name}
                                onChange={(e) => handleChange("responsibles", [{ ...formData.responsibles[0], first_name: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Prénom du/de la responsable"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.responsibles[0].last_name}
                                onChange={(e) => handleChange("responsibles", [{ ...formData.responsibles[0], last_name: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Nom du/de la responsable"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="tel"
                                value={formData.responsibles[0].phone}
                                onChange={(e) => handleChange("responsibles", [{ ...formData.responsibles[0], phone: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Téléphone du/de la responsable"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="email"
                                value={formData.responsibles[0].email}
                                onChange={(e) => handleChange("responsibles", [{ ...formData.responsibles[0], email: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Email du/de la responsable"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.responsibles[0].role}
                                onChange={(e) => handleChange("responsibles", [{ ...formData.responsibles[0], role: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Rôle dans la crèche (ex: gestionnaire, éducateur référent...)"
                            />
                        </label>
                    </div>
                )}
                {currentStep === 3 && (
                    <div className="space-y-2">
                        <label className="block">
                            <input
                                type="text"
                                value={formData.needs[0].searched_position}
                                onChange={(e) => handleChange("needs", [{ ...formData.needs[0], searched_position: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Poste recherché (ex: auxiliaire de puériculture)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.needs[0].replacement_reason}
                                onChange={(e) => handleChange("needs", [{ ...formData.needs[0], replacement_reason: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Motif du remplacement (ex: congé maternité)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.needs[0].estimated_duration}
                                onChange={(e) => handleChange("needs", [{ ...formData.needs[0], estimated_duration: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Durée estimée du besoin (ex: 3 mois)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.needs[0].availability_periods}
                                onChange={(e) => handleChange("needs", [{ ...formData.needs[0], availability_periods: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Période souhaitée (ex: mai à juillet)"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.needs[0].hours_per_week}
                                onChange={(e) => handleChange("needs", [{ ...formData.needs[0], hours_per_week: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Nombre d’heures/semaine"
                            />
                        </label>
                        <label className="block">
                            <textarea
                                value={formData.needs[0].main_tasks}
                                onChange={(e) => handleChange("needs", [{ ...formData.needs[0], main_tasks: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Missions principales"
                            />
                        </label>
                        <label className="block">
                            <textarea
                                value={formData.needs[0].required_skills}
                                onChange={(e) => handleChange("needs", [{ ...formData.needs[0], required_skills: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Compétences attendues"
                            />
                        </label>
                        <label className="block">
                            <input
                                type="text"
                                value={formData.needs[0].suggested_salary}
                                onChange={(e) => handleChange("needs", [{ ...formData.needs[0], suggested_salary: e.target.value }])}
                                className="w-full border p-2 rounded"
                                placeholder="Rémunération proposée (ex: 13€/h)"
                            />
                        </label>
                    </div>
                )}

                {/*Composant permettant d'interagir avec le forumulaire (retour, suivant, envoyer) */}
                <StepControls
                    currentStep={currentStep}
                    totalSteps={steps.length}
                    onNext={handleNext}
                    onBack={handleBack}
                    onSubmit={handleSubmit}
                />
            </div>
        </form>

    );
}
