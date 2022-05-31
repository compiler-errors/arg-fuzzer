
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3252(_: S3, _: S4, _: S5) {}
        
        fn test3252() { foo3252(S1, S2, S4, S6); }
    