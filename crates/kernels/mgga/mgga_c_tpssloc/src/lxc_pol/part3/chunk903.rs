//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 903/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk903<F: Float>(t109: F, t12808: F, t656: F, t12747: F, t12750: F, t12752: F, t12754: F, t12758: F, t12761: F, t64: F, t9358: F, t9359: F, t9361: F, t9363: F, t1268: F, t12724: F, t12725: F, t12728: F, t12734: F, t12739: F, t1458: F, t2314: F, t2363: F, t4028: F, t4072: F, t5113: F, t671: F, t9348: F) -> (F, F) {
    let t110 = 1.0 < t109;
    let t12809 = t656 * t12808;
    let t12812 = -t9358 - 22.0 / 9.0 * t9359 - 2.0 / 3.0 * t9361 + t9363 / 3.0 - 11.0 / 9.0 * t12747 - t12750 + t12752 - 3.0 / 4.0 * t64 * t12754 + t64 * t12758 / 2.0 + t64 * t12761 / 4.0 - t64 * t12809 / 8.0;
    let t12813 = piecewise3(t110, 0.0, t12812);
    let t12816 = 2.0 * t1268 * t12813 + 4.0 * t12725 * t671 + 4.0 * t12734 * t1458 + 2.0 * t12739 * t1458 + 2.0 * t1458 * t9348 + 4.0 * t2314 * t4072 + 2.0 * t2363 * t4028 + 4.0 * t4072 * t5113 + t12724 + 2.0 * t12728;
    (t12813, t12816)
}
