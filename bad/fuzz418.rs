
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo418(_: S1, _: S3, _: S4, _: S6) {}
        
        fn test418() { foo418(S7, S5, S2, S1, S8, S6, S4, S2); }
    