//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1422/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1422<F: Float>(t63332: F, t63334: F, t63888: F, t63893: F, t63911: F, t71142: F, t71144: F, t71146: F, t71152: F, t71154: F, t71156: F, t71408: F, t78002: F, t78005: F) -> F {
    let t78162 = F::cast_from(0.39862222222222222223e1_f64) * t78002 - F::cast_from(0.59793333333333333333e0_f64) * t78005 - F::cast_from(0.5314962962962962963e0_f64) * t63332 + F::cast_from(0.79724444444444444446e0_f64) * t63334 - F::cast_from(0.18257037037037037037e0_f64) * t63888 + F::cast_from(0.10954222222222222222e1_f64) * t63893 + F::cast_from(0.79724444444444444444e0_f64) * t71142 - F::cast_from(0.23917333333333333334e1_f64) * t71144 + F::cast_from(0.54771111111111111111e0_f64) * t63911 - F::cast_from(0.21908444444444444444e0_f64) * t71408 - F::cast_from(0.44291358024691358024e0_f64) * t71146 - F::cast_from(0.23917333333333333333e1_f64) * t71152 - F::cast_from(0.39862222222222222223e0_f64) * t71154 + F::cast_from(0.15944888888888888889e1_f64) * t71156;
    t78162
}
