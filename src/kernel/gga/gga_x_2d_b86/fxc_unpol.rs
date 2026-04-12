//! GGA_X_2D_B86 fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b86.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_PI, M_SQRT2};
use crate::math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_2d_b86_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = f64::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = f64::sqrt(zeta_threshold);
        let t13 = f64::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t17 = M_SQRT2;
        let t18 = 1.0 / t3 * t15 * t17;
        let t19 = f64::sqrt(rho[ip]);
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t23 = sigma[ip] / t21;
        let t25 = 1.0 + 0.421e-2 * t23;
        let t28 = 1.0 + 0.238e-3 * t23;
        let t29 = 1.0 / t28;
        let t33 = piecewise3(t2, 0.0, -2.0 / 3.0 * t18 * t19 * t25 * t29);
        let tzk0 = 2.0 * t33;
        zk[ip] += tzk0;
        let t39 = t15 * t17;
        let t41 = 1.0 / t19 / t21;
        let t47 = t28 * t28;
        let t48 = 1.0 / t47;
        let t50 = t25 * t48 * sigma[ip];
        let t54 = piecewise3(t2, 0.0, -t18 / t19 * t25 * t29 / 3.0 + 0.47504762934721079361e-2 * t39 * t41 * sigma[ip] * t29 - 0.26855424176873199259e-3 * t39 * t41 * t50);
        let tvrho0 = 2.0 * rho[ip] * t54 + 2.0 * t33;
        vrho[ip] += tvrho0;
        let t58 = 1.0 / t19 / t20;
        let t62 = t58 * t25;
        let t67 = piecewise3(t2, 0.0, -0.15834920978240359787e-2 * t39 * t58 * t29 + 0.8951808058957733086e-4 * t39 * t62 * t48);
        let tvsigma0 = 2.0 * rho[ip] * t67;
        vsigma[ip] += tvsigma0;
        let t76 = t20 * t20;
        let t78 = 1.0 / t19 / t76;
        let t86 = t76 * t21;
        let t88 = 1.0 / t19 / t86;
        let t89 = sigma[ip] * sigma[ip];
        let t94 = t39 * t88;
        let t96 = 1.0 / t47 / t28;
        let t97 = t25 * t96;
        let t98 = t97 * t89;
        let t102 = piecewise3(t2, 0.0, t18 / t19 / rho[ip] * t25 * t29 / 6.0 - 0.14251428880416323808e-1 * t39 * t78 * sigma[ip] * t29 + 0.80566272530619597777e-3 * t39 * t78 * t50 + 0.67836801470781701328e-5 * t39 * t88 * t89 * t48 - 0.38349545724574928542e-6 * t94 * t98);
        let tv2rho20 = 2.0 * rho[ip] * t102 + 4.0 * t54;
        v2rho2[ip] += tv2rho20;
        let t108 = t76 * t20;
        let t110 = 1.0 / t19 / t108;
        let t111 = t110 * t48;
        let t115 = t41 * t25;
        let t119 = t39 * t110;
        let t120 = t97 * sigma[ip];
        let t124 = piecewise3(t2, 0.0, 0.39587302445600899468e-2 * t39 * t41 * t29 - 0.22612267156927233776e-5 * t39 * t111 * sigma[ip] - 0.22379520147394332715e-3 * t39 * t115 * t48 + 0.12783181908191642847e-6 * t119 * t120);
        let tv2rhosigma0 = 2.0 * rho[ip] * t124 + 2.0 * t67;
        v2rhosigma[ip] += tv2rhosigma0;
        let t127 = t76 * rho[ip];
        let t129 = 1.0 / t19 / t127;
        let t133 = t129 * t25;
        let t138 = piecewise3(t2, 0.0, 0.75374223856424112585e-6 * t39 * t129 * t48 - 0.42610606360638809489e-7 * t39 * t133 * t96);
        let tv2sigma20 = 2.0 * rho[ip] * t138;
        v2sigma2[ip] += tv2sigma20;
    }
}
