
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4359(_: S2, _: S4) {}
        
        fn test4359() { foo4359(S3, S8, S2, S1); }
    