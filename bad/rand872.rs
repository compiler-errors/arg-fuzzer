
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo872(_: S7, _: S3, _: S7, _: S1) {}
        
        fn test872() { foo872(S5, S8, S4, S2, S3, S7, S6, S1); }
    