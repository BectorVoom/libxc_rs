//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 768/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk768<F: Float>(t1174: F, t4916: F, t1714: F, t3448: F, t3451: F, t3295: F, t3464: F, t4770: F, t4773: F, t4776: F, t4779: F, t457: F, t460: F, t974: F, t1184: F, t1180: F, t1187: F, t3430: F, t3433: F, t3436: F, t3447: F, t4887: F, t4889: F, t4897: F, t4901: F, t4905: F, t4909: F, t4913: F) -> (F, F, F, F, F, F) {
    let t4917 = t1174 * t4916;
    let t4919 = t3448 * t1714;
    let t4920 = t4919 * t3451;
    let t4928 = -t3464 + t3295 / 9.0 + t4770 / 9.0 + t4773 / 18.0 - t4776 / 3.0 - t4779 / 6.0;
    let t4929 = t457 * t4928;
    let t4930 = t4929 * t460;
    let t4931 = t974 * t4930;
    let t4934 = t974 * t457;
    let t4935 = t1714 * t1184;
    let t4936 = t4935 * t460;
    let t4937 = t4934 * t4936;
    let t4940 = -0.74074074074074074073e-3 * t4887 + 0.74074074074074074073e-3 * t4889 * t1180 + 0.22222222222222222222e-2 * t4889 * t1187 - t3430 - 0.9259259259259259259e-4 * t3433 - 0.27777777777777777777e-3 * t3436 - 0.9259259259259259259e-4 * t4897 + 0.37037037037037037036e-3 * t3447 * t4901 + 0.27777777777777777777e-3 * t3447 * t4905 - 0.55555555555555555554e-3 * t3447 * t4909 - 0.27777777777777777777e-3 * t1174 * t4913 - 0.27777777777777777777e-3 * t4917 + 0.27777777777777777777e-3 * t3447 * t4920 - 0.83333333333333333332e-3 * t1174 * t4931 - 0.83333333333333333332e-3 * t1174 * t4937;
    (t4919, t4928, t4930, t4934, t4936, t4940)
}
