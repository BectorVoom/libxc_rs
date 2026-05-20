//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1064/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1064<F: Float>(t21870: F, t21885: F, t1137: F, t11352: F, t21854: F, t1671: F, t6020: F, t3264: F, t1129: F, t11350: F, t11420: F, t15146: F, t1683: F, t1695: F, t18840: F, t18899: F, t21726: F, t21728: F, t21812: F, t21815: F, t21836: F, t21839: F, t21842: F, t21845: F, t21855: F, t3332: F, t3357: F, t3376: F, t3401: F, t4797: F, t6053: F, t6056: F) -> (F, F, F, F, F, F) {
    let t21886 = t21870 + t21885;
    let t21887 = t21886 * t1137;
    let t21890 = t21854 * t11352;
    let t21895 = t1671 * t6020;
    let t21897 = F::new(6.0) * t3264 * t21895;
    let t21898 = -t21726 + t21728 - t21812 - t21815 - F::cast_from(0.35089341735807877242e1_f64) * t3376 * t21836 + F::cast_from(0.51947577317044391277e2_f64) * t3401 * t21839 - F::new(6.0) * t3332 * t21842 + F::cast_from(0.96491876992155210402e2_f64) * t3357 * t21845 + F::new(3.0) * t18840 * t1683 + F::new(3.0) * t4797 * t6053 + F::cast_from(0.96491876992155210402e2_f64) * t15146 * t6056 - F::cast_from(0.19298375398431042081e3_f64) * t11420 * t21855 + F::new(1.0) * t1129 * t21887 + F::cast_from(0.2069040516770936012e4_f64) * t11350 * t21890 + F::cast_from(0.17544670867903938621e1_f64) * t18899 * t1695 + t21897;
    (t21886, t21887, t21890, t21895, t21897, t21898)
}
