//! `GgaFunctional` enum: enumerates the compiled GGA functionals routable
//! by `dispatch_gga`.
//!
//! Each variant maps one libxc functional ID to a kernel module in one of the
//! 58 `crates/kernel-gga-*` sub-crates. Variants are ordered by libxc ID for
//! consistency with the registry.
//!
//! The `GgaXLb` variant (libxc id 160) is the vxc-only special case: it has no
//! `exc_unpol`/`exc_pol` kernels in `crates/kernel-gga-8d`, so `has_exc()` returns
//! `false` and the dispatch layer rejects `DerivativeOrder::Exc` requests for it.
//!
//! Non-compiled GGA functionals (e.g. `gga_c_gam` id 33, or functionals whose
//! kernel modules have only partial derivative coverage after the recent
//! monolith-splitting work) are NOT represented here — `GgaFunctional::from_id`
//! returns `Err(UnsupportedFunctional)` for them.
//!
//! **Note on `gga_x_herman`:** libxc id 104 is on the "removed" list in libxc
//! 7.0.0 (`xc_funcs_removed.h`). Although `crates/kernel-gga-22/src/gga_x_herman`
//! exists with a full 10-arm kernel, its ID cannot pass through
//! `FunctionalId::from_raw(104)` (it returns `RemovedFunctionalId`). The variant
//! is therefore intentionally absent from this enum.
//!
//! **Note on template kernels:** Eleven kernel modules are "templates" whose
//! single source backs multiple libxc IDs through varying ext-params defaults
//! (e.g. `gga_x_vmt` backs both `gga_x_vmt_ge`=70 and `gga_x_vmt_pbe`=71). For
//! Phase 4 we route each template to its *primary* libxc ID (70 for vmt). Other
//! IDs backed by the same template remain `UnsupportedFunctional` until a
//! follow-up plan adds per-variant parameter plumbing. The mapping list is
//! captured in `tools/generate_gga_roster.py::TEMPLATE_ID_OVERRIDES`.

use crate::error::LibxcRsError;
use crate::model::FunctionalId;

/// Enumerates the 105 compiled, routable GGA functionals `dispatch_gga` can serve.
///
/// Variants are ordered by libxc ID (ascending) for consistency with the registry.
/// The one vxc-only case (`GgaXLb`) is flagged via `has_exc() == false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GgaFunctional {
    /// `gga_x_hcth_a` (libxc id 34, kernel batch 21)
    GgaXHcthA,
    /// `gga_x_ev93` (libxc id 35, kernel batch 19)
    GgaXEv93,
    /// `gga_x_q2d` (libxc id 48, kernel batch 18)
    GgaXQ2d,
    /// `gga_k_tflw` (libxc id 52, kernel batch 22)
    GgaKTflw,
    /// `gga_k_apbeint` (libxc id 54, kernel batch 20)
    GgaKApbeint,
    /// `gga_x_ak13` (libxc id 56, kernel batch 20)
    GgaXAk13,
    /// `gga_k_meyer` (libxc id 57, kernel batch 18)
    GgaKMeyer,
    /// `gga_x_lv_rpw86` (libxc id 58, kernel batch 19)
    GgaXLvRpw86,
    /// `gga_x_pbeint` (libxc id 60, kernel batch 20)
    GgaXPbeint,
    /// `gga_x_vmt84` (libxc id 68, kernel batch 18)
    GgaXVmt84,
    /// `gga_x_vmt` (libxc id 70, kernel batch 21)
    GgaXVmt,
    /// `gga_x_n12` (libxc id 82, kernel batch 17)
    GgaXN12,
    /// `gga_c_op_xalpha` (libxc id 84, kernel batch 21)
    GgaCOpXalpha,
    /// `gga_c_op_g96` (libxc id 85, kernel batch 17)
    GgaCOpG96,
    /// `gga_c_op_pbe` (libxc id 86, kernel batch 16)
    GgaCOpPbe,
    /// `gga_c_op_b88` (libxc id 87, kernel batch 16)
    GgaCOpB88,
    /// `gga_x_ssb_sw` (libxc id 90, kernel batch 20)
    GgaXSsbSw,
    /// `gga_x_bpccac` (libxc id 98, kernel batch 17)
    GgaXBpccac,
    /// `gga_c_tca` (libxc id 100, kernel batch 19)
    GgaCTca,
    /// `gga_x_pbe` (libxc id 101, kernel batch 22)
    GgaXPbe,
    /// `gga_x_b86` (libxc id 103, kernel batch 21)
    GgaXB86,
    /// `gga_x_b88` (libxc id 106, kernel batch 21)
    GgaXB88,
    /// `gga_x_g96` (libxc id 107, kernel batch 22)
    GgaXG96,
    /// `gga_x_pw86` (libxc id 108, kernel batch 21)
    GgaXPw86,
    /// `gga_x_pw91` (libxc id 109, kernel batch 15)
    GgaXPw91,
    /// `gga_x_optx` (libxc id 110, kernel batch 12)
    GgaXOptx,
    /// `gga_x_dk87` (libxc id 111, kernel batch 14)
    GgaXDk87,
    /// `gga_x_lg93` (libxc id 113, kernel batch 18)
    GgaXLg93,
    /// `gga_x_rpbe` (libxc id 117, kernel batch 22)
    GgaXRpbe,
    /// `gga_x_wc` (libxc id 118, kernel batch 20)
    GgaXWc,
    /// `gga_x_am05` (libxc id 120, kernel batch 17)
    GgaXAm05,
    /// `gga_x_pbea` (libxc id 121, kernel batch 22)
    GgaXPbea,
    /// `gga_x_mpbe` (libxc id 122, kernel batch 20)
    GgaXMpbe,
    /// `gga_x_2d_b86_mgc` (libxc id 124, kernel batch 22)
    GgaX2dB86Mgc,
    /// `gga_x_bayesian` (libxc id 125, kernel batch 20)
    GgaXBayesian,
    /// `gga_x_2d_b88` (libxc id 127, kernel batch 21)
    GgaX2dB88,
    /// `gga_x_2d_b86` (libxc id 128, kernel batch 21)
    GgaX2dB86,
    /// `gga_x_2d_pbe` (libxc id 129, kernel batch 22)
    GgaX2dPbe,
    /// `gga_c_pbe` (libxc id 130, kernel batch 4g)
    GgaCPbe,
    /// `gga_c_lyp` (libxc id 131, kernel batch 18)
    GgaCLyp,
    /// `gga_c_p86` (libxc id 132, kernel batch 18)
    GgaCP86,
    /// `gga_c_am05` (libxc id 135, kernel batch 18)
    GgaCAm05,
    /// `gga_c_lm` (libxc id 137, kernel batch 19)
    GgaCLm,
    /// `gga_x_rge2` (libxc id 142, kernel batch 22)
    GgaXRge2,
    /// `gga_x_kt` (libxc id 145, kernel batch 20)
    GgaXKt,
    /// `gga_c_wl` (libxc id 147, kernel batch 19)
    GgaCWl,
    /// `gga_c_wi` (libxc id 148, kernel batch 21)
    GgaCWi,
    /// `gga_x_sogga11` (libxc id 151, kernel batch 18)
    GgaXSogga11,
    /// `gga_xc_th1` (libxc id 154, kernel batch 18)
    GgaXcTh1,
    /// `gga_xc_th2` (libxc id 155, kernel batch 19)
    GgaXcTh2,
    /// `gga_xc_th3` (libxc id 156, kernel batch 18)
    GgaXcTh3,
    /// `gga_x_c09x` (libxc id 158, kernel batch 22)
    GgaXC09x,
    /// `gga_x_lb` (libxc id 160, kernel batch 8d) — vxc-only (no exc kernels)
    GgaXLb,
    /// `gga_x_lspbe` (libxc id 168, kernel batch 22)
    GgaXLspbe,
    /// `gga_x_lsrpbe` (libxc id 169, kernel batch 22)
    GgaXLsrpbe,
    /// `gga_x_ncap` (libxc id 180, kernel batch 17)
    GgaXNcap,
    /// `gga_x_ol2` (libxc id 183, kernel batch 22)
    GgaXOl2,
    /// `gga_k_apbe` (libxc id 185, kernel batch 22)
    GgaKApbe,
    /// `gga_x_htbs` (libxc id 191, kernel batch 19)
    GgaXHtbs,
    /// `gga_x_airy` (libxc id 192, kernel batch 19)
    GgaXAiry,
    /// `gga_x_lag` (libxc id 193, kernel batch 20)
    GgaXLag,
    /// `gga_c_pbe_vwn` (libxc id 216, kernel batch 14)
    GgaCPbeVwn,
    /// `gga_k_rational_p` (libxc id 218, kernel batch 21)
    GgaKRationalP,
    /// `gga_k_pg` (libxc id 219, kernel batch 22)
    GgaKPg,
    /// `gga_c_p86vwn` (libxc id 252, kernel batch 16)
    GgaCP86vwn,
    /// `gga_c_op_pw91` (libxc id 262, kernel batch 16)
    GgaCOpPw91,
    /// `gga_x_cap` (libxc id 270, kernel batch 20)
    GgaXCap,
    /// `gga_c_bmk` (libxc id 280, kernel batch 6d)
    GgaCBmk,
    /// `gga_x_beefvdw` (libxc id 285, kernel batch 17)
    GgaXBeefvdw,
    /// `gga_x_pbetrans` (libxc id 291, kernel batch 19)
    GgaXPbetrans,
    /// `gga_x_chachiyo` (libxc id 298, kernel batch 13)
    GgaXChachiyo,
    /// `gga_c_chachiyo` (libxc id 309, kernel batch 20)
    GgaCChachiyo,
    /// `gga_c_ccdf` (libxc id 313, kernel batch 17)
    GgaCCcdf,
    /// `hyb_gga_xc_case21` (libxc id 390, kernel batch 15)
    HybGgaXcCase21,
    /// `gga_x_s12` (libxc id 495, kernel batch 20)
    GgaXS12,
    /// `gga_k_pearson` (libxc id 511, kernel batch 22)
    GgaKPearson,
    /// `gga_k_ol1` (libxc id 512, kernel batch 22)
    GgaKOl1,
    /// `gga_k_ol2` (libxc id 513, kernel batch 22)
    GgaKOl2,
    /// `gga_k_pw86` (libxc id 515, kernel batch 21)
    GgaKPw86,
    /// `gga_k_dk` (libxc id 516, kernel batch 19)
    GgaKDk,
    /// `gga_k_lc94` (libxc id 521, kernel batch 19)
    GgaKLc94,
    /// `gga_k_llp` (libxc id 522, kernel batch 21)
    GgaKLlp,
    /// `gga_k_thakkar` (libxc id 523, kernel batch 20)
    GgaKThakkar,
    /// `gga_x_ityh` (libxc id 529, kernel batch 15)
    GgaXItyh,
    /// `gga_x_sfat` (libxc id 530, kernel batch 14)
    GgaXSfat,
    /// `gga_x_sg4` (libxc id 533, kernel batch 20)
    GgaXSg4,
    /// `gga_x_gg99` (libxc id 535, kernel batch 5g)
    GgaXGg99,
    /// `gga_x_pbepow` (libxc id 539, kernel batch 21)
    GgaXPbepow,
    /// `gga_c_scan_e0` (libxc id 553, kernel batch 15)
    GgaCScanE0,
    /// `gga_c_w94` (libxc id 561, kernel batch 21)
    GgaCW94,
    /// `gga_c_cs1` (libxc id 565, kernel batch 18)
    GgaCCs1,
    /// `gga_k_exp4` (libxc id 597, kernel batch 22)
    GgaKExp4,
    /// `gga_x_sfat_pbe` (libxc id 601, kernel batch 14)
    GgaXSfatPbe,
    /// `gga_x_fd_lb94` (libxc id 604, kernel batch 21)
    GgaXFdLb94,
    /// `gga_k_lkt` (libxc id 613, kernel batch 21)
    GgaKLkt,
    /// `gga_k_mpbe` (libxc id 616, kernel batch 20)
    GgaKMpbe,
    /// `gga_k_vt84f` (libxc id 619, kernel batch 17)
    GgaKVt84f,
    /// `gga_k_lgap` (libxc id 620, kernel batch 21)
    GgaKLgap,
    /// `gga_x_ityh_optx` (libxc id 622, kernel batch 16)
    GgaXItyhOptx,
    /// `gga_x_ityh_pbe` (libxc id 623, kernel batch 15)
    GgaXItyhPbe,
    /// `gga_c_lypr` (libxc id 624, kernel batch 16)
    GgaCLypr,
    /// `gga_k_lgap_ge` (libxc id 633, kernel batch 22)
    GgaKLgapGe,
    /// `hyb_gga_x_cam_s12` (libxc id 646, kernel batch 15)
    HybGgaXCamS12,
    /// `gga_x_pbe_erf_gws` (libxc id 655, kernel batch 17)
    GgaXPbeErfGws,
    /// `gga_x_q1d` (libxc id 734, kernel batch 19)
    GgaXQ1d,
}

impl GgaFunctional {
    /// Map a libxc functional ID to a dispatchable GGA variant, or return an error.
    ///
    /// Returns `Err(UnsupportedFunctional)` for:
    /// - GGA functionals whose kernel module does not yet exist in
    ///   `crates/kernel-gga-*` (most of libxc's 256 GGA IDs).
    /// - GGA kernel modules with only partial derivative coverage (e.g.
    ///   `gga_c_ft97`, `gga_c_optc`, `gga_c_sg4`) — these have translated
    ///   files for some (order, spin) pairs but are missing arms that would
    ///   be needed for correct dispatch.
    /// - Non-GGA IDs (route those via `dispatch_lda` / `dispatch_mgga`).
    /// - Removed IDs such as `gga_x_herman` (id 104), which cannot even pass
    ///   through `FunctionalId::from_raw`.
    pub fn from_id(id: FunctionalId) -> Result<Self, LibxcRsError> {
        match id.raw() {
            34 => Ok(Self::GgaXHcthA),
            35 => Ok(Self::GgaXEv93),
            48 => Ok(Self::GgaXQ2d),
            52 => Ok(Self::GgaKTflw),
            54 => Ok(Self::GgaKApbeint),
            56 => Ok(Self::GgaXAk13),
            57 => Ok(Self::GgaKMeyer),
            58 => Ok(Self::GgaXLvRpw86),
            60 => Ok(Self::GgaXPbeint),
            68 => Ok(Self::GgaXVmt84),
            70 => Ok(Self::GgaXVmt),
            82 => Ok(Self::GgaXN12),
            84 => Ok(Self::GgaCOpXalpha),
            85 => Ok(Self::GgaCOpG96),
            86 => Ok(Self::GgaCOpPbe),
            87 => Ok(Self::GgaCOpB88),
            90 => Ok(Self::GgaXSsbSw),
            98 => Ok(Self::GgaXBpccac),
            100 => Ok(Self::GgaCTca),
            101 => Ok(Self::GgaXPbe),
            103 => Ok(Self::GgaXB86),
            106 => Ok(Self::GgaXB88),
            107 => Ok(Self::GgaXG96),
            108 => Ok(Self::GgaXPw86),
            109 => Ok(Self::GgaXPw91),
            110 => Ok(Self::GgaXOptx),
            111 => Ok(Self::GgaXDk87),
            113 => Ok(Self::GgaXLg93),
            117 => Ok(Self::GgaXRpbe),
            118 => Ok(Self::GgaXWc),
            120 => Ok(Self::GgaXAm05),
            121 => Ok(Self::GgaXPbea),
            122 => Ok(Self::GgaXMpbe),
            124 => Ok(Self::GgaX2dB86Mgc),
            125 => Ok(Self::GgaXBayesian),
            127 => Ok(Self::GgaX2dB88),
            128 => Ok(Self::GgaX2dB86),
            129 => Ok(Self::GgaX2dPbe),
            130 => Ok(Self::GgaCPbe),
            131 => Ok(Self::GgaCLyp),
            132 => Ok(Self::GgaCP86),
            135 => Ok(Self::GgaCAm05),
            137 => Ok(Self::GgaCLm),
            142 => Ok(Self::GgaXRge2),
            145 => Ok(Self::GgaXKt),
            147 => Ok(Self::GgaCWl),
            148 => Ok(Self::GgaCWi),
            151 => Ok(Self::GgaXSogga11),
            154 => Ok(Self::GgaXcTh1),
            155 => Ok(Self::GgaXcTh2),
            156 => Ok(Self::GgaXcTh3),
            158 => Ok(Self::GgaXC09x),
            160 => Ok(Self::GgaXLb),
            168 => Ok(Self::GgaXLspbe),
            169 => Ok(Self::GgaXLsrpbe),
            180 => Ok(Self::GgaXNcap),
            183 => Ok(Self::GgaXOl2),
            185 => Ok(Self::GgaKApbe),
            191 => Ok(Self::GgaXHtbs),
            192 => Ok(Self::GgaXAiry),
            193 => Ok(Self::GgaXLag),
            216 => Ok(Self::GgaCPbeVwn),
            218 => Ok(Self::GgaKRationalP),
            219 => Ok(Self::GgaKPg),
            252 => Ok(Self::GgaCP86vwn),
            262 => Ok(Self::GgaCOpPw91),
            270 => Ok(Self::GgaXCap),
            280 => Ok(Self::GgaCBmk),
            285 => Ok(Self::GgaXBeefvdw),
            291 => Ok(Self::GgaXPbetrans),
            298 => Ok(Self::GgaXChachiyo),
            309 => Ok(Self::GgaCChachiyo),
            313 => Ok(Self::GgaCCcdf),
            390 => Ok(Self::HybGgaXcCase21),
            495 => Ok(Self::GgaXS12),
            511 => Ok(Self::GgaKPearson),
            512 => Ok(Self::GgaKOl1),
            513 => Ok(Self::GgaKOl2),
            515 => Ok(Self::GgaKPw86),
            516 => Ok(Self::GgaKDk),
            521 => Ok(Self::GgaKLc94),
            522 => Ok(Self::GgaKLlp),
            523 => Ok(Self::GgaKThakkar),
            529 => Ok(Self::GgaXItyh),
            530 => Ok(Self::GgaXSfat),
            533 => Ok(Self::GgaXSg4),
            535 => Ok(Self::GgaXGg99),
            539 => Ok(Self::GgaXPbepow),
            553 => Ok(Self::GgaCScanE0),
            561 => Ok(Self::GgaCW94),
            565 => Ok(Self::GgaCCs1),
            597 => Ok(Self::GgaKExp4),
            601 => Ok(Self::GgaXSfatPbe),
            604 => Ok(Self::GgaXFdLb94),
            613 => Ok(Self::GgaKLkt),
            616 => Ok(Self::GgaKMpbe),
            619 => Ok(Self::GgaKVt84f),
            620 => Ok(Self::GgaKLgap),
            622 => Ok(Self::GgaXItyhOptx),
            623 => Ok(Self::GgaXItyhPbe),
            624 => Ok(Self::GgaCLypr),
            633 => Ok(Self::GgaKLgapGe),
            646 => Ok(Self::HybGgaXCamS12),
            655 => Ok(Self::GgaXPbeErfGws),
            734 => Ok(Self::GgaXQ1d),
            _ => Err(LibxcRsError::UnsupportedFunctional {
                id,
                reason: "GGA functional not yet translated into crates/kernel-gga*",
            }),
        }
    }

    /// Reverse of `from_id`: return the canonical libxc functional ID for this variant.
    pub fn to_id(self) -> FunctionalId {
        let raw = match self {
            Self::GgaXHcthA => 34,
            Self::GgaXEv93 => 35,
            Self::GgaXQ2d => 48,
            Self::GgaKTflw => 52,
            Self::GgaKApbeint => 54,
            Self::GgaXAk13 => 56,
            Self::GgaKMeyer => 57,
            Self::GgaXLvRpw86 => 58,
            Self::GgaXPbeint => 60,
            Self::GgaXVmt84 => 68,
            Self::GgaXVmt => 70,
            Self::GgaXN12 => 82,
            Self::GgaCOpXalpha => 84,
            Self::GgaCOpG96 => 85,
            Self::GgaCOpPbe => 86,
            Self::GgaCOpB88 => 87,
            Self::GgaXSsbSw => 90,
            Self::GgaXBpccac => 98,
            Self::GgaCTca => 100,
            Self::GgaXPbe => 101,
            Self::GgaXB86 => 103,
            Self::GgaXB88 => 106,
            Self::GgaXG96 => 107,
            Self::GgaXPw86 => 108,
            Self::GgaXPw91 => 109,
            Self::GgaXOptx => 110,
            Self::GgaXDk87 => 111,
            Self::GgaXLg93 => 113,
            Self::GgaXRpbe => 117,
            Self::GgaXWc => 118,
            Self::GgaXAm05 => 120,
            Self::GgaXPbea => 121,
            Self::GgaXMpbe => 122,
            Self::GgaX2dB86Mgc => 124,
            Self::GgaXBayesian => 125,
            Self::GgaX2dB88 => 127,
            Self::GgaX2dB86 => 128,
            Self::GgaX2dPbe => 129,
            Self::GgaCPbe => 130,
            Self::GgaCLyp => 131,
            Self::GgaCP86 => 132,
            Self::GgaCAm05 => 135,
            Self::GgaCLm => 137,
            Self::GgaXRge2 => 142,
            Self::GgaXKt => 145,
            Self::GgaCWl => 147,
            Self::GgaCWi => 148,
            Self::GgaXSogga11 => 151,
            Self::GgaXcTh1 => 154,
            Self::GgaXcTh2 => 155,
            Self::GgaXcTh3 => 156,
            Self::GgaXC09x => 158,
            Self::GgaXLb => 160,
            Self::GgaXLspbe => 168,
            Self::GgaXLsrpbe => 169,
            Self::GgaXNcap => 180,
            Self::GgaXOl2 => 183,
            Self::GgaKApbe => 185,
            Self::GgaXHtbs => 191,
            Self::GgaXAiry => 192,
            Self::GgaXLag => 193,
            Self::GgaCPbeVwn => 216,
            Self::GgaKRationalP => 218,
            Self::GgaKPg => 219,
            Self::GgaCP86vwn => 252,
            Self::GgaCOpPw91 => 262,
            Self::GgaXCap => 270,
            Self::GgaCBmk => 280,
            Self::GgaXBeefvdw => 285,
            Self::GgaXPbetrans => 291,
            Self::GgaXChachiyo => 298,
            Self::GgaCChachiyo => 309,
            Self::GgaCCcdf => 313,
            Self::HybGgaXcCase21 => 390,
            Self::GgaXS12 => 495,
            Self::GgaKPearson => 511,
            Self::GgaKOl1 => 512,
            Self::GgaKOl2 => 513,
            Self::GgaKPw86 => 515,
            Self::GgaKDk => 516,
            Self::GgaKLc94 => 521,
            Self::GgaKLlp => 522,
            Self::GgaKThakkar => 523,
            Self::GgaXItyh => 529,
            Self::GgaXSfat => 530,
            Self::GgaXSg4 => 533,
            Self::GgaXGg99 => 535,
            Self::GgaXPbepow => 539,
            Self::GgaCScanE0 => 553,
            Self::GgaCW94 => 561,
            Self::GgaCCs1 => 565,
            Self::GgaKExp4 => 597,
            Self::GgaXSfatPbe => 601,
            Self::GgaXFdLb94 => 604,
            Self::GgaKLkt => 613,
            Self::GgaKMpbe => 616,
            Self::GgaKVt84f => 619,
            Self::GgaKLgap => 620,
            Self::GgaXItyhOptx => 622,
            Self::GgaXItyhPbe => 623,
            Self::GgaCLypr => 624,
            Self::GgaKLgapGe => 633,
            Self::HybGgaXCamS12 => 646,
            Self::GgaXPbeErfGws => 655,
            Self::GgaXQ1d => 734,
        };
        FunctionalId::from_raw(raw)
            .expect("GgaFunctional::to_id produced a registry-valid id")
    }

    /// True when this functional has `exc_unpol`/`exc_pol` kernels.
    ///
    /// Currently only `GgaXLb` (libxc id 160) returns `false`: its kernel
    /// module in `crates/kernel-gga-8d` has only `{vxc,fxc,kxc,lxc}_{unpol,pol}`
    /// — no energy-density kernels.
    pub fn has_exc(self) -> bool {
        !matches!(self, Self::GgaXLb)
    }

    /// Kernel module name (matches the directory under `crates/kernel-gga-*/src/`).
    pub fn kernel_name(self) -> &'static str {
        match self {
            Self::GgaXHcthA => "gga_x_hcth_a",
            Self::GgaXEv93 => "gga_x_ev93",
            Self::GgaXQ2d => "gga_x_q2d",
            Self::GgaKTflw => "gga_k_tflw",
            Self::GgaKApbeint => "gga_k_apbeint",
            Self::GgaXAk13 => "gga_x_ak13",
            Self::GgaKMeyer => "gga_k_meyer",
            Self::GgaXLvRpw86 => "gga_x_lv_rpw86",
            Self::GgaXPbeint => "gga_x_pbeint",
            Self::GgaXVmt84 => "gga_x_vmt84",
            Self::GgaXVmt => "gga_x_vmt",
            Self::GgaXN12 => "gga_x_n12",
            Self::GgaCOpXalpha => "gga_c_op_xalpha",
            Self::GgaCOpG96 => "gga_c_op_g96",
            Self::GgaCOpPbe => "gga_c_op_pbe",
            Self::GgaCOpB88 => "gga_c_op_b88",
            Self::GgaXSsbSw => "gga_x_ssb_sw",
            Self::GgaXBpccac => "gga_x_bpccac",
            Self::GgaCTca => "gga_c_tca",
            Self::GgaXPbe => "gga_x_pbe",
            Self::GgaXB86 => "gga_x_b86",
            Self::GgaXB88 => "gga_x_b88",
            Self::GgaXG96 => "gga_x_g96",
            Self::GgaXPw86 => "gga_x_pw86",
            Self::GgaXPw91 => "gga_x_pw91",
            Self::GgaXOptx => "gga_x_optx",
            Self::GgaXDk87 => "gga_x_dk87",
            Self::GgaXLg93 => "gga_x_lg93",
            Self::GgaXRpbe => "gga_x_rpbe",
            Self::GgaXWc => "gga_x_wc",
            Self::GgaXAm05 => "gga_x_am05",
            Self::GgaXPbea => "gga_x_pbea",
            Self::GgaXMpbe => "gga_x_mpbe",
            Self::GgaX2dB86Mgc => "gga_x_2d_b86_mgc",
            Self::GgaXBayesian => "gga_x_bayesian",
            Self::GgaX2dB88 => "gga_x_2d_b88",
            Self::GgaX2dB86 => "gga_x_2d_b86",
            Self::GgaX2dPbe => "gga_x_2d_pbe",
            Self::GgaCPbe => "gga_c_pbe",
            Self::GgaCLyp => "gga_c_lyp",
            Self::GgaCP86 => "gga_c_p86",
            Self::GgaCAm05 => "gga_c_am05",
            Self::GgaCLm => "gga_c_lm",
            Self::GgaXRge2 => "gga_x_rge2",
            Self::GgaXKt => "gga_x_kt",
            Self::GgaCWl => "gga_c_wl",
            Self::GgaCWi => "gga_c_wi",
            Self::GgaXSogga11 => "gga_x_sogga11",
            Self::GgaXcTh1 => "gga_xc_th1",
            Self::GgaXcTh2 => "gga_xc_th2",
            Self::GgaXcTh3 => "gga_xc_th3",
            Self::GgaXC09x => "gga_x_c09x",
            Self::GgaXLb => "gga_x_lb",
            Self::GgaXLspbe => "gga_x_lspbe",
            Self::GgaXLsrpbe => "gga_x_lsrpbe",
            Self::GgaXNcap => "gga_x_ncap",
            Self::GgaXOl2 => "gga_x_ol2",
            Self::GgaKApbe => "gga_k_apbe",
            Self::GgaXHtbs => "gga_x_htbs",
            Self::GgaXAiry => "gga_x_airy",
            Self::GgaXLag => "gga_x_lag",
            Self::GgaCPbeVwn => "gga_c_pbe_vwn",
            Self::GgaKRationalP => "gga_k_rational_p",
            Self::GgaKPg => "gga_k_pg",
            Self::GgaCP86vwn => "gga_c_p86vwn",
            Self::GgaCOpPw91 => "gga_c_op_pw91",
            Self::GgaXCap => "gga_x_cap",
            Self::GgaCBmk => "gga_c_bmk",
            Self::GgaXBeefvdw => "gga_x_beefvdw",
            Self::GgaXPbetrans => "gga_x_pbetrans",
            Self::GgaXChachiyo => "gga_x_chachiyo",
            Self::GgaCChachiyo => "gga_c_chachiyo",
            Self::GgaCCcdf => "gga_c_ccdf",
            Self::HybGgaXcCase21 => "hyb_gga_xc_case21",
            Self::GgaXS12 => "gga_x_s12",
            Self::GgaKPearson => "gga_k_pearson",
            Self::GgaKOl1 => "gga_k_ol1",
            Self::GgaKOl2 => "gga_k_ol2",
            Self::GgaKPw86 => "gga_k_pw86",
            Self::GgaKDk => "gga_k_dk",
            Self::GgaKLc94 => "gga_k_lc94",
            Self::GgaKLlp => "gga_k_llp",
            Self::GgaKThakkar => "gga_k_thakkar",
            Self::GgaXItyh => "gga_x_ityh",
            Self::GgaXSfat => "gga_x_sfat",
            Self::GgaXSg4 => "gga_x_sg4",
            Self::GgaXGg99 => "gga_x_gg99",
            Self::GgaXPbepow => "gga_x_pbepow",
            Self::GgaCScanE0 => "gga_c_scan_e0",
            Self::GgaCW94 => "gga_c_w94",
            Self::GgaCCs1 => "gga_c_cs1",
            Self::GgaKExp4 => "gga_k_exp4",
            Self::GgaXSfatPbe => "gga_x_sfat_pbe",
            Self::GgaXFdLb94 => "gga_x_fd_lb94",
            Self::GgaKLkt => "gga_k_lkt",
            Self::GgaKMpbe => "gga_k_mpbe",
            Self::GgaKVt84f => "gga_k_vt84f",
            Self::GgaKLgap => "gga_k_lgap",
            Self::GgaXItyhOptx => "gga_x_ityh_optx",
            Self::GgaXItyhPbe => "gga_x_ityh_pbe",
            Self::GgaCLypr => "gga_c_lypr",
            Self::GgaKLgapGe => "gga_k_lgap_ge",
            Self::HybGgaXCamS12 => "hyb_gga_x_cam_s12",
            Self::GgaXPbeErfGws => "gga_x_pbe_erf_gws",
            Self::GgaXQ1d => "gga_x_q1d",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_id_routes_gga_x_pbe() {
        let id = FunctionalId::from_raw(101).unwrap();
        assert_eq!(GgaFunctional::from_id(id).unwrap(), GgaFunctional::GgaXPbe);
    }

    #[test]
    fn from_id_routes_gga_c_pbe() {
        let id = FunctionalId::from_raw(130).unwrap();
        assert_eq!(GgaFunctional::from_id(id).unwrap(), GgaFunctional::GgaCPbe);
    }

    #[test]
    fn from_id_routes_gga_x_lb_vxc_only() {
        let id = FunctionalId::from_raw(160).unwrap();
        let f = GgaFunctional::from_id(id).unwrap();
        assert_eq!(f, GgaFunctional::GgaXLb);
        assert!(!f.has_exc(), "gga_x_lb must report has_exc() == false");
    }

    #[test]
    fn from_id_rejects_lda_x_id() {
        // LDA_X (id 1) is valid in the registry but is not a GGA.
        let id = FunctionalId::from_raw(1).unwrap();
        let err = GgaFunctional::from_id(id).unwrap_err();
        match err {
            LibxcRsError::UnsupportedFunctional { id: e_id, reason } => {
                assert_eq!(e_id.raw(), 1);
                assert!(reason.contains("not yet translated"), "reason: {reason}");
            }
            other => panic!("expected UnsupportedFunctional, got {other:?}"),
        }
    }

    #[test]
    fn from_id_rejects_non_compiled_gga_id() {
        // gga_c_gam (id 33) is a valid registry entry but has no
        // kernel module in crates/kernel-gga-*.
        let id = FunctionalId::from_raw(33).unwrap();
        let err = GgaFunctional::from_id(id).unwrap_err();
        assert!(matches!(err, LibxcRsError::UnsupportedFunctional { .. }));
    }

    #[test]
    fn has_exc_only_false_for_gga_x_lb() {
        assert!(!GgaFunctional::GgaXLb.has_exc());
        assert!(GgaFunctional::GgaXPbe.has_exc());
        assert!(GgaFunctional::GgaCPbe.has_exc());
        assert!(GgaFunctional::HybGgaXcCase21.has_exc());
    }

    #[test]
    fn to_id_round_trips_through_from_id() {
        let cases = [
            (101, GgaFunctional::GgaXPbe),
            (130, GgaFunctional::GgaCPbe),
            (160, GgaFunctional::GgaXLb),
            (646, GgaFunctional::HybGgaXCamS12),
            (734, GgaFunctional::GgaXQ1d),
        ];
        for (raw, variant) in cases {
            let id = FunctionalId::from_raw(raw).unwrap();
            assert_eq!(GgaFunctional::from_id(id).unwrap(), variant);
            assert_eq!(variant.to_id().raw(), raw);
        }
    }

    #[test]
    fn kernel_name_is_libxc_name() {
        assert_eq!(GgaFunctional::GgaXPbe.kernel_name(), "gga_x_pbe");
        assert_eq!(GgaFunctional::GgaXLb.kernel_name(), "gga_x_lb");
        assert_eq!(GgaFunctional::HybGgaXCamS12.kernel_name(), "hyb_gga_x_cam_s12");
    }
}
