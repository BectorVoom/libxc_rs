//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 916/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk916<F: Float>(t5: F, t32110: F, t7687: F, t1458: F, t8774: F, t15899: F, t8808: F, t1441: F, t8717: F, t3701: F, t7939: F, t2095: F, t32245: F, t32249: F, t32257: F, t32258: F, t33103: F, t33107: F, t33111: F, t33119: F, t8707: F) -> (F, F, F, F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t33878 = t32110 * t7687;
    let t33883 = t8774 * t1458;
    let t33886 = t8808 * t15899;
    let t33893 = t1441 * t8717;
    let t33899 = t3701 * t7939;
    let t33900 = t2095 * t33899;
    let t33915 = piecewise3::<f64>(t8, F::new(0.0), F::new(5.0) / F::new(36.0) * t33103 * t8707 - F::new(5.0) / F::new(6.0) * t32245 * t33107 - F::new(5.0) / F::new(9.0) * t32249 * t33111 - t32257 + F::new(5.0) / F::new(18.0) * t32258 * t33119);
    (t33878, t33883, t33886, t33893, t33899, t33900, t33915)
}
