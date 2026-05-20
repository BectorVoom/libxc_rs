//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2621/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2621<F: Float>(t5181: F, t591: F, t16465: F, t225: F, t12344: F, t5234: F, t1369: F, t16336: F, t3876: F, t16333: F, t3866: F, t1831: F, t40284: F) -> (F, F, F, F, F, F, F) {
    let t53852 = F::new(16.0) * t5181 * t591;
    let t53866 = t16465 * t225;
    let t53880 = t5234 * t12344;
    let t53881 = t53880 * t1369;
    let t53883 = t16336 * t3876;
    let t53893 = t3866 * t16333;
    let t53895 = t40284 * t1831;
    (t53852, t53866, t53880, t53881, t53883, t53893, t53895)
}
