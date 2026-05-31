//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1387/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1387<F: Float>(t1509: F, t8543: F, t1888: F, t232: F, t6646: F, t92552: F, t26676: F, t33384: F, t6547: F, t118578: F, t112778: F, t112784: F, t112804: F, t118533: F, t118535: F, t118539: F, t118546: F, t118549: F, t118552: F, t118556: F, t118559: F, t118562: F, t118566: F, t118569: F, t118573: F, t118576: F, t118580: F) -> (F, F, F, F, F) {
    let t121553 = t8543 * t1509;
    let t121560 = t1888 * t6646 * t92552 * t232;
    let t121563 = t1888 * t6646 * t26676;
    let t121574 = t6547 * t33384;
    let t121591 = F::cast_from(0.11304371706359309439e-1_f64) * t118578;
    let t121593 = -t118533 / F::cast_from(768.0_f64) - t118535 / F::cast_from(768.0_f64) - t118539 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t118546 - F::cast_from(0.16149102437656156341e-2_f64) * t118549 + F::cast_from(0.67826230238155856632e-1_f64) * t118552 + F::cast_from(0.26915170729426927235e-3_f64) * t112778 + F::cast_from(0.32298204875312312682e-2_f64) * t118556 + F::cast_from(0.96894614625936938046e-2_f64) * t118559 + F::cast_from(0.67826230238155856634e-1_f64) * t112784 + t118562 / F::cast_from(384.0_f64) + t112804 + F::cast_from(0.96894614625936938046e-2_f64) * t118566 - F::cast_from(0.16149102437656156341e-2_f64) * t118569 + F::cast_from(0.16149102437656156341e-2_f64) * t118573 + t118576 / F::cast_from(768.0_f64) + t121591 + F::cast_from(0.67826230238155856632e-1_f64) * t118580;
    (t121553, t121560, t121563, t121574, t121593)
}
