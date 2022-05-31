
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14532(_: S2, _: S5, _: S3) {}
        
        fn test14532() { foo14532(S6, S2, S1, S0, S7); }
    