
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16289(_: S5, _: S1, _: S6) {}
        
        fn test16289() { foo16289(S2, S7, S0, S5, S2, S2); }
    