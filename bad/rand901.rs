
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo901(_: S1, _: S5, _: S6, _: S8) {}
        
        fn test901() { foo901(S2, S5, S5, S1, S5, S4); }
    