// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use crate::schema::*;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, TimeZone, Utc};
use diesel::backend::Backend;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Integer;
use serde::Serialize;
use std::convert::TryInto;
use uuid::Uuid;
use ya_model::payment as api_model;
use ya_persistence::types::BigDecimalField;

#[derive(Queryable, Debug, Identifiable, Insertable)]
#[table_name = "pay_allocation"]
pub struct NewAllocation {
    pub id: String,
    pub total_amount: BigDecimalField,
    pub timeout: Option<NaiveDateTime>,
    pub make_deposit: bool,
}

impl From<api_model::NewAllocation> for NewAllocation {
    fn from(allocation: api_model::NewAllocation) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            total_amount: allocation.total_amount.into(),
            timeout: allocation.timeout.map(|v| v.naive_utc()),
            make_deposit: allocation.make_deposit,
        }
    }
}

pub struct Allocation {
    pub allocation: NewAllocation,
    pub spent_amount: BigDecimal,
    pub remaining_amount: BigDecimal,
}

impl From<Allocation> for api_model::Allocation {
    fn from(allocation: Allocation) -> Self {
        Self {
            allocation_id: allocation.allocation.id,
            total_amount: allocation.allocation.total_amount.into(),
            spent_amount: allocation.spent_amount,
            remaining_amount: allocation.remaining_amount,
            timeout: allocation
                .allocation
                .timeout
                .map(|v| Utc.from_utc_datetime(&v)),
            make_deposit: allocation.allocation.make_deposit,
        }
    }
}

#[derive(Queryable, QueryableByName, Debug, Identifiable, Insertable)]
#[table_name = "pay_debit_note"]
pub struct DebitNote {
    pub id: String,
    pub issuer_id: String,
    pub recipient_id: String,
    pub previous_debit_note_id: Option<String>,
    pub agreement_id: String,
    pub activity_id: Option<String>,
    pub status: String,
    pub timestamp: NaiveDateTime,
    pub total_amount_due: BigDecimalField,
    pub usage_counter_vector: Option<Vec<u8>>,
    pub credit_account_id: String,
    pub payment_platform: Option<String>,
    pub payment_due_date: Option<NaiveDateTime>,
}

impl From<api_model::DebitNote> for DebitNote {
    fn from(debit_note: api_model::DebitNote) -> Self {
        Self {
            id: debit_note.debit_note_id,
            issuer_id: debit_note.issuer_id,
            recipient_id: debit_note.recipient_id,
            previous_debit_note_id: debit_note.previous_debit_note_id,
            agreement_id: debit_note.agreement_id,
            activity_id: debit_note.activity_id,
            status: debit_note.status.into(),
            timestamp: debit_note.timestamp.naive_utc(),
            total_amount_due: debit_note.total_amount_due.into(),
            usage_counter_vector: debit_note
                .usage_counter_vector
                .map(|v| v.to_string().into_bytes()),
            credit_account_id: debit_note.credit_account_id,
            payment_platform: debit_note.payment_platform,
            payment_due_date: debit_note.payment_due_date.map(|d| d.naive_utc()),
        }
    }
}

impl From<DebitNote> for api_model::DebitNote {
    fn from(debit_note: DebitNote) -> Self {
        api_model::DebitNote {
            debit_note_id: debit_note.id,
            issuer_id: debit_note.issuer_id,
            recipient_id: debit_note.recipient_id,
            previous_debit_note_id: debit_note.previous_debit_note_id,
            timestamp: Utc.from_utc_datetime(&debit_note.timestamp),
            agreement_id: debit_note.agreement_id,
            activity_id: debit_note.activity_id,
            total_amount_due: debit_note.total_amount_due.into(),
            usage_counter_vector: debit_note
                .usage_counter_vector
                .map(|v| serde_json::from_str(&String::from_utf8(v).unwrap()).unwrap()),
            credit_account_id: debit_note.credit_account_id,
            payment_platform: debit_note.payment_platform,
            payment_due_date: debit_note
                .payment_due_date
                .map(|d| Utc.from_utc_datetime(&d)),
            status: debit_note.status.into(),
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "pay_debit_note"]
pub struct NewDebitNote {
    pub id: String,
    pub issuer_id: String,
    pub recipient_id: String,
    pub previous_debit_note_id: Option<String>,
    pub agreement_id: String,
    pub activity_id: Option<String>,
    pub total_amount_due: BigDecimalField,
    pub usage_counter_vector: Option<Vec<u8>>,
    pub credit_account_id: String,
    pub payment_platform: Option<String>,
    pub payment_due_date: Option<NaiveDateTime>,
}

impl NewDebitNote {
    pub fn from_api_model(
        debit_note: api_model::NewDebitNote,
        issuer_id: String,
        recipient_id: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            issuer_id,
            recipient_id,
            previous_debit_note_id: None,
            agreement_id: debit_note.agreement_id,
            activity_id: debit_note.activity_id,
            total_amount_due: debit_note.total_amount_due.into(),
            usage_counter_vector: debit_note
                .usage_counter_vector
                .map(|v| v.to_string().into_bytes()),
            credit_account_id: debit_note.credit_account_id,
            payment_platform: debit_note.payment_platform,
            payment_due_date: debit_note.payment_due_date.map(|d| d.naive_utc()),
        }
    }
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "pay_debit_note_event"]
#[primary_key(debit_note_id, event_type)]
pub struct DebitNoteEvent {
    pub debit_note_id: String,
    pub event_type: String,
    pub timestamp: NaiveDateTime,
    pub details: Option<String>,
}

impl From<DebitNoteEvent> for api_model::DebitNoteEvent {
    fn from(event: DebitNoteEvent) -> Self {
        Self {
            debit_note_id: event.debit_note_id,
            timestamp: Utc.from_utc_datetime(&event.timestamp),
            details: event.details.map(|s| serde_json::from_str(&s).unwrap()),
            event_type: event.event_type.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Identifiable, Insertable)]
#[table_name = "pay_debit_note_event"]
#[primary_key(debit_note_id, event_type)]
pub struct NewDebitNoteEvent {
    pub debit_note_id: String,
    pub event_type: String,
    pub details: Option<String>,
}

impl From<api_model::NewDebitNoteEvent> for NewDebitNoteEvent {
    fn from(event: api_model::NewDebitNoteEvent) -> Self {
        Self {
            debit_note_id: event.debit_note_id,
            event_type: event.event_type.into(),
            details: event.details.map(|s| serde_json::to_string(&s).unwrap()),
        }
    }
}

#[derive(Queryable, QueryableByName, Debug, Identifiable, Insertable)]
#[table_name = "pay_invoice"]
pub struct BareInvoice {
    pub id: String,
    pub issuer_id: String,
    pub recipient_id: String,
    pub last_debit_note_id: Option<String>,
    pub agreement_id: String,
    pub status: String,
    pub timestamp: NaiveDateTime,
    pub amount: BigDecimalField,
    pub usage_counter_vector: Option<Vec<u8>>,
    pub credit_account_id: String,
    pub payment_platform: Option<String>,
    pub payment_due_date: NaiveDateTime,
}

// Because Diesel doesn't support collections of associated objects :(
#[derive(Debug)]
pub struct Invoice {
    pub invoice: BareInvoice,
    pub activity_ids: Vec<String>,
}

impl From<api_model::Invoice> for Invoice {
    fn from(invoice: api_model::Invoice) -> Self {
        Invoice {
            invoice: BareInvoice {
                id: invoice.invoice_id,
                issuer_id: invoice.issuer_id,
                recipient_id: invoice.recipient_id,
                last_debit_note_id: invoice.last_debit_note_id,
                agreement_id: invoice.agreement_id,
                status: invoice.status.into(),
                timestamp: invoice.timestamp.naive_utc(),
                amount: invoice.amount.into(),
                usage_counter_vector: invoice
                    .usage_counter_vector
                    .map(|v| v.to_string().into_bytes()),
                credit_account_id: invoice.credit_account_id,
                payment_platform: invoice.payment_platform,
                payment_due_date: invoice.payment_due_date.naive_utc(),
            },
            activity_ids: invoice.activity_ids,
        }
    }
}

impl From<Invoice> for api_model::Invoice {
    fn from(invoice_with_activity_ids: Invoice) -> Self {
        let invoice = invoice_with_activity_ids.invoice;
        let activity_ids = invoice_with_activity_ids.activity_ids;
        api_model::Invoice {
            invoice_id: invoice.id,
            issuer_id: invoice.issuer_id,
            recipient_id: invoice.recipient_id,
            last_debit_note_id: invoice.last_debit_note_id,
            timestamp: Utc.from_utc_datetime(&invoice.timestamp),
            agreement_id: invoice.agreement_id,
            activity_ids,
            amount: invoice.amount.into(),
            usage_counter_vector: invoice
                .usage_counter_vector
                .map(|v| serde_json::from_str(&String::from_utf8(v).unwrap()).unwrap()),
            credit_account_id: invoice.credit_account_id,
            payment_platform: invoice.payment_platform,
            payment_due_date: Utc.from_utc_datetime(&invoice.payment_due_date),
            status: invoice.status.into(),
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "pay_invoice"]
pub struct BareNewInvoice {
    pub id: String,
    pub issuer_id: String,
    pub recipient_id: String,
    pub last_debit_note_id: Option<String>,
    pub agreement_id: String,
    pub amount: BigDecimalField,
    pub usage_counter_vector: Option<Vec<u8>>,
    pub credit_account_id: String,
    pub payment_platform: Option<String>,
    pub payment_due_date: NaiveDateTime,
}

// Because Diesel doesn't support collections of associated objects :(
#[derive(Debug)]
pub struct NewInvoice {
    pub invoice: BareNewInvoice,
    pub activity_ids: Vec<String>,
}

impl NewInvoice {
    pub fn from_api_model(
        invoice: api_model::NewInvoice,
        issuer_id: String,
        recipient_id: String,
    ) -> Self {
        Self {
            invoice: BareNewInvoice {
                id: Uuid::new_v4().to_string(),
                issuer_id,
                recipient_id,
                last_debit_note_id: None,
                agreement_id: invoice.agreement_id,
                amount: invoice.amount.into(),
                usage_counter_vector: invoice
                    .usage_counter_vector
                    .map(|v| v.to_string().into_bytes()),
                credit_account_id: invoice.credit_account_id,
                payment_platform: invoice.payment_platform,
                payment_due_date: invoice.payment_due_date.naive_utc(),
            },
            activity_ids: invoice.activity_ids.unwrap_or(vec![]),
        }
    }
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "pay_invoice_event"]
#[primary_key(invoice_id, event_type)]
pub struct InvoiceEvent {
    pub invoice_id: String,
    pub event_type: String,
    pub timestamp: NaiveDateTime,
    pub details: Option<String>,
}

impl From<InvoiceEvent> for api_model::InvoiceEvent {
    fn from(event: InvoiceEvent) -> Self {
        Self {
            invoice_id: event.invoice_id,
            timestamp: Utc.from_utc_datetime(&event.timestamp),
            details: event.details.map(|s| serde_json::from_str(&s).unwrap()),
            event_type: event.event_type.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Identifiable, Insertable)]
#[table_name = "pay_invoice_event"]
#[primary_key(invoice_id, event_type)]
pub struct NewInvoiceEvent {
    pub invoice_id: String,
    pub event_type: String,
    pub details: Option<String>,
}

impl From<api_model::NewInvoiceEvent> for NewInvoiceEvent {
    fn from(event: api_model::NewInvoiceEvent) -> Self {
        Self {
            invoice_id: event.invoice_id,
            event_type: event.event_type.into(),
            details: event.details.map(|s| serde_json::to_string(&s).unwrap()),
        }
    }
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "pay_invoice_event_type"]
#[primary_key(event_type)]
pub struct InvoiceEventType {
    pub event_type: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "pay_invoice_status"]
#[primary_key(status)]
pub struct InvoiceStatus {
    pub status: String,
}

#[derive(Queryable, Debug, Identifiable, Insertable)]
#[table_name = "pay_invoice_x_activity"]
#[primary_key(invoice_id, activity_id)]
pub struct InvoiceXActivity {
    pub invoice_id: String,
    pub activity_id: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "pay_payment"]
pub struct BarePayment {
    pub id: String,
    pub payer_id: String,
    pub payee_id: String,
    pub amount: BigDecimalField,
    pub timestamp: NaiveDateTime,
    pub allocation_id: Option<String>,
    pub details: Vec<u8>,
}

#[derive(Debug)]
pub struct Payment {
    pub payment: BarePayment,
    pub debit_note_ids: Vec<String>,
    pub invoice_ids: Vec<String>,
}

impl From<api_model::Payment> for Payment {
    fn from(payment: api_model::Payment) -> Self {
        Self {
            payment: BarePayment {
                id: payment.payment_id,
                payer_id: payment.payer_id,
                payee_id: payment.payee_id,
                amount: payment.amount.into(),
                timestamp: payment.timestamp.naive_utc(),
                allocation_id: payment.allocation_id,
                details: base64::decode(&payment.details).unwrap(),
            },
            debit_note_ids: payment.debit_note_ids.unwrap_or(vec![]),
            invoice_ids: payment.invoice_ids.unwrap_or(vec![]),
        }
    }
}

impl From<Payment> for api_model::Payment {
    fn from(payment: Payment) -> Self {
        Self {
            payment_id: payment.payment.id,
            payer_id: payment.payment.payer_id,
            payee_id: payment.payment.payee_id,
            amount: payment.payment.amount.into(),
            timestamp: Utc.from_utc_datetime(&payment.payment.timestamp),
            allocation_id: payment.payment.allocation_id,
            debit_note_ids: Some(payment.debit_note_ids),
            invoice_ids: Some(payment.invoice_ids),
            details: base64::encode(&payment.payment.details),
        }
    }
}

#[derive(Debug, Identifiable, Insertable)]
#[table_name = "pay_payment"]
pub struct BareNewPayment {
    pub id: String,
    pub payer_id: String,
    pub payee_id: String,
    pub amount: BigDecimalField,
    pub allocation_id: Option<String>,
    pub details: Vec<u8>,
}

impl From<BarePayment> for BareNewPayment {
    fn from(payment: BarePayment) -> Self {
        Self {
            id: payment.id,
            payer_id: payment.payer_id,
            payee_id: payment.payee_id,
            amount: payment.amount,
            allocation_id: None,
            details: payment.details,
        }
    }
}

#[derive(Debug)]
pub struct NewPayment {
    pub payment: BareNewPayment,
    pub debit_note_ids: Vec<String>,
    pub invoice_ids: Vec<String>,
}

impl From<Payment> for NewPayment {
    fn from(payment: Payment) -> Self {
        Self {
            payment: payment.payment.into(),
            debit_note_ids: payment.debit_note_ids,
            invoice_ids: payment.invoice_ids,
        }
    }
}

#[derive(Queryable, Debug, Identifiable, Insertable)]
#[table_name = "pay_payment_x_debit_note"]
#[primary_key(payment_id, debit_note_id)]
pub struct PaymentXDebitNote {
    pub payment_id: String,
    pub debit_note_id: String,
}

#[derive(Queryable, Debug, Identifiable, Insertable)]
#[table_name = "pay_payment_x_invoice"]
#[primary_key(payment_id, invoice_id)]
pub struct PaymentXInvoice {
    pub payment_id: String,
    pub invoice_id: String,
}
