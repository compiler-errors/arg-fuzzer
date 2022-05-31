
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8875(_: S7, _: S2) {}
        
        fn test8875() { foo8875(S1, S7, S4, S3, S4, S3); }
    