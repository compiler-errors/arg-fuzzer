
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10658(_: S4, _: S7) {}
        
        fn test10658() { foo10658(S1, S2, S4, S5); }
    