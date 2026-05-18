//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 841/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk841<F: Float>(t457: F, t4928: F, t460: F, t974: F, t1184: F, t1714: F, t1174: F, t1180: F, t1187: F, t3430: F, t3433: F, t3436: F, t3447: F, t4887: F, t4889: F, t4897: F, t4901: F, t4905: F, t4909: F, t4913: F, t4917: F, t4920: F) -> (F, F, F, F) {
    let t4929 = t457 * t4928;
    let t4930 = t4929 * t460;
    let t4931 = t974 * t4930;
    let t4934 = t974 * t457;
    let t4935 = t1714 * t1184;
    let t4936 = t4935 * t460;
    let t4937 = t4934 * t4936;
    let t4940 = -F::new(0.74074074074074074073e-3) * t4887 + F::new(0.74074074074074074073e-3) * t4889 * t1180 + F::new(0.22222222222222222222e-2) * t4889 * t1187 - t3430 - F::new(0.9259259259259259259e-4) * t3433 - F::new(0.27777777777777777777e-3) * t3436 - F::new(0.9259259259259259259e-4) * t4897 + F::new(0.37037037037037037036e-3) * t3447 * t4901 + F::new(0.27777777777777777777e-3) * t3447 * t4905 - F::new(0.55555555555555555554e-3) * t3447 * t4909 - F::new(0.27777777777777777777e-3) * t1174 * t4913 - F::new(0.27777777777777777777e-3) * t4917 + F::new(0.27777777777777777777e-3) * t3447 * t4920 - F::new(0.83333333333333333332e-3) * t1174 * t4931 - F::new(0.83333333333333333332e-3) * t1174 * t4937;
    (t4930, t4934, t4936, t4940)
}
