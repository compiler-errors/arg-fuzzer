
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3097(_: S6, _: S4) {}
        
        fn test3097() { foo3097(S2, S5, S6, S7); }
    