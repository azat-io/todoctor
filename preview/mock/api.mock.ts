import { defineMock } from 'vite-plugin-mock-dev-server'

import type { Data } from '~/typings/index.d'

export default defineMock({
  body: <Data>{
    data: [
      {
        blame: {
          summary: 'Initial implementation of lab results module',
          email: 'sarah.lee@todoctor.app',
          date: '2017-07-18T12:10:47Z',
          author: 'Sarah Lee',
          commit: 'q1r2s3t',
        },
        comment: 'TODO: Add integration with external lab result APIs',
        path: 'src/modules/labResults/labResults.ts',
        line: 45,
      },
      {
        blame: {
          summary: 'Refactored prescription reminders',
          email: 'jessica.williams@todoctor.app',
          date: '2018-12-25T13:24:17Z',
          author: 'Jessica Williams',
          commit: 't4u5v6w',
        },
        comment:
          'TODO: Add test coverage for edge cases in prescription reminders',
        path: 'src/services/reminders/prescriptionReminders.ts',
        line: 59,
      },
      {
        blame: {
          summary: 'Added export feature for patient data',
          email: 'sarah.lee@todoctor.app',
          date: '2018-05-09T09:15:44Z',
          author: 'Sarah Lee',
          commit: 'b4c5d6e',
        },
        path: 'src/modules/patientData/exportPatientData.ts',
        comment: 'TODO: Implement secure PDF export',
        line: 67,
      },
      {
        blame: {
          summary: 'Added two-factor authentication for login',
          email: 'john.doe@todoctor.app',
          date: '2019-10-12T10:00:34Z',
          author: 'John Doe',
          commit: 'a4b5c6d',
        },
        comment: 'TODO: Implement backup codes for 2FA',
        path: 'src/modules/auth/twoFactorAuth.ts',
        line: 65,
      },
      {
        blame: {
          summary: 'Updated appointment booking flow',
          email: 'michael.johnson@todoctor.app',
          date: '2019-09-21T09:12:10Z',
          author: 'Michael Johnson',
          commit: 'x7y8z9a',
        },
        comment: 'TODO: Refactor appointment notifications to support SMS',
        path: 'src/services/appointments/bookingFlow.ts',
        line: 47,
      },
      {
        blame: {
          email: 'michael.johnson@todoctor.app',
          summary: 'Refactored billing system',
          date: '2020-11-02T09:45:22Z',
          author: 'Michael Johnson',
          commit: 'n1o2p3q',
        },
        comment: 'TODO: Update payment methods to include Apple Pay',
        path: 'src/services/billing/billingSystem.ts',
        line: 89,
      },
      {
        blame: {
          summary: 'Fixed notification delays in appointment reminders',
          email: 'jessica.williams@todoctor.app',
          date: '2022-01-03T16:45:23Z',
          author: 'Jessica Williams',
          commit: 'x6y7z8a',
        },
        comment: 'TODO: Add priority levels for urgent notifications',
        path: 'src/services/notifications/appointmentReminders.ts',
        line: 49,
      },
      {
        blame: {
          summary: 'Improved accessibility for appointment booking',
          email: 'daniel.davis@todoctor.app',
          date: '2021-12-22T07:33:15Z',
          author: 'Daniel Davis',
          commit: 'f6g7h8i',
        },
        comment: 'TODO: Add screen reader support for calendar view',
        path: 'src/components/appointments/calendarView.ts',
        line: 39,
      },
      {
        blame: {
          summary: 'Fixed patient data sync issue',
          email: 'john.doe@todoctor.app',
          date: '2023-11-15T10:23:45Z',
          author: 'John Doe',
          commit: 'a1b2c3d',
        },
        comment: 'TODO: Ensure patient history is encrypted before storage',
        path: 'src/modules/patientData/patientHistory.ts',
        line: 34,
      },
      {
        blame: {
          summary: 'Refined analytics dashboard for doctors',
          email: 'michael.johnson@todoctor.app',
          date: '2023-04-20T10:12:05Z',
          author: 'Michael Johnson',
          commit: 'y1z2a3b',
        },
        comment: 'TODO: Add customizable metrics for patient health stats',
        path: 'src/components/analytics/doctorDashboard.ts',
        line: 128,
      },
      {
        blame: {
          summary: 'Enhanced patient dashboard UI',
          email: 'emily.smith@todoctor.app',
          date: '2023-08-18T17:43:12Z',
          author: 'Emily Smith',
          commit: 'k8l9m0n',
        },
        comment: 'TODO: Improve responsive design for mobile devices',
        path: 'src/components/dashboard/patientDashboard.ts',
        line: 102,
      },
      {
        blame: {
          summary: 'Integrated telemedicine functionality',
          email: 'daniel.davis@todoctor.app',
          date: '2021-03-14T14:33:11Z',
          author: 'Daniel Davis',
          commit: 'u9v0w1x',
        },
        comment: 'TODO: Support video call recording for patient consultations',
        path: 'src/modules/telemedicine/videoConsultation.ts',
        line: 95,
      },
      {
        blame: {
          summary: 'Improved data validation for prescriptions',
          email: 'sarah.lee@todoctor.app',
          date: '2024-01-15T11:33:28Z',
          author: 'Sarah Lee',
          commit: 'b9c8d7e',
        },
        comment: 'TODO: Add additional checks for drug interactions',
        path: 'src/modules/prescriptions/validatePrescription.ts',
        line: 90,
      },
      {
        blame: {
          summary: 'Added vaccine tracking module',
          email: 'emily.smith@todoctor.app',
          date: '2022-04-05T14:58:22Z',
          author: 'Emily Smith',
          commit: 'd3f4g5h',
        },
        comment: 'TODO: Add notification system for upcoming vaccines',
        path: 'src/modules/vaccineTracking/vaccineReminder.ts',
        line: 120,
      },
      {
        blame: {
          summary: 'Added two-factor authentication for login',
          email: 'john.doe@todoctor.app',
          date: '2019-10-12T10:00:34Z',
          author: 'John Doe',
          commit: 'a4b5c6d',
        },
        comment: 'TODO: Implement backup codes for 2FA',
        path: 'src/modules/auth/twoFactorAuth.ts',
        line: 65,
      },
      {
        blame: {
          summary: 'Implemented data backup feature',
          email: 'jessica.williams@todoctor.app',
          date: '2024-02-12T14:58:22Z',
          author: 'Jessica Williams',
          commit: 'a8f9d1k',
        },
        comment:
          'TODO: Ensure encrypted backups for sensitive patient information',
        path: 'src/modules/dataBackup/dataBackup.ts',
        line: 78,
      },
      {
        blame: {
          summary: 'Improved performance on loading patient history',
          email: 'daniel.davis@todoctor.app',
          date: '2021-09-14T07:00:45Z',
          author: 'Daniel Davis',
          commit: 't1r2y3a',
        },
        comment: 'TODO: Optimize database queries for large datasets',
        path: 'src/services/patientHistory/patientDataLoader.ts',
        line: 155,
      },
      {
        blame: {
          summary: 'Updated patient search algorithm',
          email: 'michael.johnson@todoctor.app',
          date: '2021-08-11T08:10:17Z',
          author: 'Michael Johnson',
          commit: 'h9k0w2p',
        },
        comment: 'TODO: Add support for fuzzy search in patient database',
        path: 'src/services/search/patientSearch.ts',
        line: 150,
      },
      {
        blame: {
          summary: 'Added support for multiple insurance providers',
          email: 'emily.smith@todoctor.app',
          date: '2023-06-19T12:45:31Z',
          author: 'Emily Smith',
          commit: 'i1t2g5h',
        },
        comment: 'TODO: Validate insurance details before submission',
        path: 'src/services/insurance/insuranceProvider.ts',
        line: 97,
      },
      {
        blame: {
          summary: 'Implemented data backup feature',
          email: 'jessica.williams@todoctor.app',
          date: '2024-02-12T14:58:22Z',
          author: 'Jessica Williams',
          commit: 'a8f9d1k',
        },
        comment:
          'TODO: Ensure encrypted backups for sensitive patient information',
        path: 'src/modules/dataBackup/dataBackup.ts',
        line: 78,
      },
    ],
    name: 'awesome-project-name',
    currentPath: '/#',
    version: '0.1.0',
  },
  headers: {
    'Content-Type': 'application/json',
  },
  url: '/data.json',
})
