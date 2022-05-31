
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15649(_: S3, _: S6) {}
        
        fn test15649() { foo15649(S1, S2, S3, S4, S5, S7, S8); }
    