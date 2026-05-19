//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1293/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1293<F: Float>(t5: F, t125855: F, t125900: F, t112: F, t671: F, t8859: F, t117773: F, t120125: F, t120127: F, t120129: F, t120131: F, t120137: F, t120140: F, t120165: F, t123084: F, t123086: F, t125100: F, t1458: F, t32609: F, t4072: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t125902 = piecewise3::<F>(t8, F::new(0.0), t125855 + t125900);
    let t125903 = t125902 * t112;
    let t125910 = t8859 * t671;
    let t125915 = F::new(2.0) * t117773 * t1458 + F::new(2.0) * t125100 * t671 + F::new(2.0) * t125910 * t1458 + F::new(2.0) * t32609 * t4072 + t120125 + t120127 + t120129 + t120131 + t120137 + t120140 + t120165 + F::new(4.0) * t123084 + F::new(4.0) * t123086 + t125903;
    (t125903, t125910, t125915)
}
