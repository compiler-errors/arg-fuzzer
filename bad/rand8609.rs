
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8609(_: S6, _: S8, _: S7) {}
        
        fn test8609() { foo8609(S5, S1, S6, S5, S2, S3, S1); }
    