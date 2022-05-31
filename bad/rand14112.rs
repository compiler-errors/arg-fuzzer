
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14112(_: S2, _: S5) {}
        
        fn test14112() { foo14112(S7, S2, S7, S1, S4, S0); }
    