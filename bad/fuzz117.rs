
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo117(_: S2, _: S6, _: S8, _: S6, _: S6, _: S7, _: S7) {}
        
        fn test117() { foo117(S5, S1, S8, S2, S8, S4, S4, S1); }
    