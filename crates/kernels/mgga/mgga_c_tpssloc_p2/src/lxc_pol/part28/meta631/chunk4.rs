//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1981/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1981<F: Float>(t87328: F, t87330: F, t87332: F, t87338: F, t87341: F, t87345: F, t87347: F, t87363: F, t87335: F, t87343: F, t87351: F, t87355: F, t87359: F, t87365: F, t87369: F, t87371: F, t87373: F, t87375: F) -> F {
    let t92645 = F::cast_from(0.80745512188280781706e-3_f64) * t87328;
    let t92646 = F::new(7.0) / F::new(144.0) * t87330;
    let t92647 = F::new(7.0) / F::new(144.0) * t87332;
    let t92649 = F::cast_from(0.13457585364713463618e-3_f64) * t87338;
    let t92650 = F::new(7.0) / F::new(144.0) * t87341;
    let t92652 = F::new(119.0) / F::new(864.0) * t87345;
    let t92653 = F::cast_from(0.11304371706359309439e-1_f64) * t87347;
    let t92657 = F::new(7.0) / F::new(288.0) * t87363;
    let t92663 = -t92645 + t92646 + t92647 - F::cast_from(0.80745512188280781706e-3_f64) * t87335 + t92649 + t92650 - t87343 / F::new(192.0) - t92652 - t92653 - F::cast_from(0.16956557559538964158e-1_f64) * t87351 - F::cast_from(0.24223653656484234512e-2_f64) * t87355 - F::cast_from(0.24223653656484234512e-2_f64) * t87359 - t92657 - F::new(5.0) / F::new(192.0) * t87365 - t87369 / F::new(128.0) + t87371 / F::new(128.0) - t87373 / F::new(768.0) - t87375 / F::new(96.0);
    t92663
}
