//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1407/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1407<F: Float>(t33428: F, t6562: F, t794: F, t114790: F, t7488: F, t114965: F, t118935: F, t118938: F, t118941: F, t118944: F, t1912: F, t2053: F, t25184: F, t25329: F, t25348: F, t2718: F, t31416: F, t33398: F, t7087: F, t7107: F, t855: F, t865: F, t92847: F, t92939: F, t98279: F) -> F {
    let t121749 = t6562 * t794 * t33428;
    let t121753 = t6562 * t114790 * t7488;
    let t121770 = -F::new(0.41123351671205660912e-2) * t121749 - t25348 * t7107 + F::new(0.41123351671205660912e-2) * t121753 + F::new(2.0) * t855 * t2718 * t33398 * t865 + F::new(2.0) * t855 * t2718 * t2053 * t25329 - t118935 - t92939 * t1912 - t118938 + t118941 + F::new(0.41123351671205660912e-2) * t114965 + F::new(2.0) * t7087 * t25184 - t92847 * t1912 - F::new(6.0) * t98279 * t31416 - t118944;
    t121770
}
